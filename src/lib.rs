use pgx::*;

pg_module_magic!();

pub struct HelloFdwState {
    rownum: usize,
}

// HACK: By making an alias of this type, pgx-macro will use this name
// which is the postgres specified name.
#[allow(non_camel_case_types)]
type oid = pgx_pg_sys::Oid;
#[allow(non_camel_case_types)]
type fdw_handler = pgx::PgBox<pgx_pg_sys::FdwRoutine>;

#[pg_extern]
fn hello_fdw_validator(_options: Vec<String>, _oid: oid) {
    debug1!("HelloFdw: hello_fdw_validator");
}

#[pg_extern]
fn hello_fdw_handler() -> fdw_handler {
    debug1!("FDW was called!");
    let mut fdwroutine =
        pgx::PgBox::<pgx_pg_sys::FdwRoutine>::alloc_node(pgx_pg_sys::NodeTag_T_FdwRoutine);

    // Set callback functions.
    fdwroutine.GetForeignRelSize = Some(hello_get_foreign_rel_size);
    fdwroutine.GetForeignPaths = Some(hello_get_foreign_paths);
    fdwroutine.GetForeignPlan = Some(hello_get_foreign_plan);
    fdwroutine.ExplainForeignScan = Some(hello_explain_foreign_scan);
    fdwroutine.BeginForeignScan = Some(hello_begin_foreign_scan);
    fdwroutine.IterateForeignScan = Some(hello_iterate_foreign_scan);
    fdwroutine.ReScanForeignScan = Some(hello_re_scan_foreign_scan);
    fdwroutine.EndForeignScan = Some(hello_end_foreign_scan);
    fdwroutine.AnalyzeForeignTable = Some(hello_analyze_foreign_table);

    debug1!("FDW init about to return!");
    fdwroutine
}

#[pg_guard]
unsafe extern "C" fn hello_get_foreign_rel_size(
    _root: *mut pgx_pg_sys::PlannerInfo,
    baserel: *mut pgx_pg_sys::RelOptInfo,
    _foreigntableid: pgx_pg_sys::Oid,
) {
    debug1!("HelloFdw: hello_get_foreign_rel_size");

    (*baserel).rows = 1.0;
    (*baserel).fdw_private = std::ptr::null_mut();
}

#[pg_guard]
unsafe extern "C" fn hello_get_foreign_paths(
    root: *mut pgx_pg_sys::PlannerInfo,
    baserel: *mut pgx_pg_sys::RelOptInfo,
    _foreigntableid: pgx_pg_sys::Oid,
) {
    debug1!("HelloFdw: hello_get_foreign_paths");
    pgx_pg_sys::add_path(
        baserel,
        create_foreignscan_path(
            root,
            baserel,
            std::ptr::null_mut(),
            (*baserel).rows,
            10.0,
            1000.0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        ),
    )
}

unsafe extern "C" fn hello_get_foreign_plan(
    _root: *mut pgx_pg_sys::PlannerInfo,
    baserel: *mut pgx_pg_sys::RelOptInfo,
    _foreigntableid: pgx_pg_sys::Oid,
    best_path: *mut pgx_pg_sys::ForeignPath,
    tlist: *mut pgx_pg_sys::List,
    mut scan_clauses: *mut pgx_pg_sys::List,
    outer_plan: *mut pgx_pg_sys::Plan,
) -> *mut pgx_pg_sys::ForeignScan {
    debug1!("HelloFdw: hello_get_foreign_plan");

    scan_clauses = pgx_pg_sys::extract_actual_clauses(scan_clauses, false);
    pgx_pg_sys::make_foreignscan(
        tlist,
        scan_clauses,
        (*baserel).relid,
        std::ptr::null_mut(),
        (*best_path).fdw_private,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        outer_plan,
    )
}

#[pg_guard]
extern "C" fn hello_explain_foreign_scan(
    _node: *mut pgx_pg_sys::ForeignScanState,
    es: *mut pgx_pg_sys::ExplainState,
) {
    debug1!("HelloFdw: hello_explain_foreign_scan");

    // TODO: Cleanup this memory.
    let hello = std::ffi::CString::new("Hello").expect("checked and works");
    let hello_explain = std::ffi::CString::new("Hello Explain Value").expect("checked and works");
    unsafe { pgx_pg_sys::ExplainPropertyText(hello.as_ptr(), hello_explain.as_ptr(), es) }
}

#[pg_guard]
unsafe extern "C" fn hello_begin_foreign_scan(
    node: *mut pgx_pg_sys::ForeignScanState,
    eflags: ::std::os::raw::c_int,
) {
    debug1!("HelloFdw: hello_begin_foreign_scan");

    if eflags & pgx_pg_sys::EXEC_FLAG_EXPLAIN_ONLY as i32 != 0 {
        debug1!("HelloFdw: hello_begin_foreign_scan: exec flag was explain only");
        return;
    }

    // TODO: palloc this in current mem context.
    let mut state = pgx::PgBox::<HelloFdwState>::alloc0();
    state.rownum = 0;
    (*node).fdw_state = state.into_pg() as *mut std::ffi::c_void;
}

#[pg_guard]
unsafe extern "C" fn hello_iterate_foreign_scan(
    node: *mut pgx_pg_sys::ForeignScanState,
) -> *mut pgx_pg_sys::TupleTableSlot {
    debug1!("HelloFdw: hello_iterate_foreign_scan");

    let slot = (*node).ss.ss_ScanTupleSlot;
    let state = (*node).fdw_state as *mut HelloFdwState;

    debug1!("HelloFdw: hello_iterate_foreign_scan: check rownum");
    debug1!(
            "HelloFdw: hello_iterate_foreign_scan: clear slot: slot: {:?}, heaptuple?: {:?}, virt? {:?}, tts_flags: {:?}",
            slot,
            (*slot).tts_ops == &pgx_pg_sys::TTSOpsHeapTuple,
            (*slot).tts_ops == &pgx_pg_sys::TTSOpsVirtual,
            (*slot).tts_flags,
        );
    if (*state).rownum > 0 {
        debug1!("HelloFdw: hello_iterate_foreign_scan: clear slot");
        let clear = (*(*slot).tts_ops).clear.unwrap();
        // Why does heap_freetuple cause this to segfault?
        //(*slot).tts_flags &= !pgx_pg_sys::TTS_FLAG_SHOULDFREE as u16;
        debug1!(
            "HelloFdw: hello_iterate_foreign_scan: clear slot: slot: {:?}, heaptuple?: {:?}, virt? {:?}, tts_flags: {:?} AFTER!",
            slot,
            (*slot).tts_ops == &pgx_pg_sys::TTSOpsHeapTuple,
            (*slot).tts_ops == &pgx_pg_sys::TTSOpsVirtual,
            (*slot).tts_flags,
        );
        clear(slot);
        debug1!("HelloFdw: hello_iterate_foreign_scan: clear slot: done");
        return slot;
    }

    debug1!("HelloFdw: hello_iterate_foreign_scan: pull rel and natts");
    let rel = (*node).ss.ss_currentRelation;
    let attinmeta = pgx_pg_sys::TupleDescGetAttInMetadata((*rel).rd_att);
    let natts = (*(*rel).rd_att).natts;

    debug1!(
        "HelloFdw: hello_iterate_foreign_scan: build values: {:?}/{}",
        (*(*attinmeta).tupdesc).natts,
        natts
    );
    let size = std::mem::size_of::<*const ::std::os::raw::c_char>() * natts as usize;
    let values = pgx_pg_sys::palloc0(size) as *mut *const ::std::os::raw::c_char;
    let slice = std::slice::from_raw_parts_mut(values, size);
    let hello_world = std::ffi::CString::new("Hello,World").expect("checked and works");
    for i in 0..natts {
        slice[i as usize] = hello_world.as_ptr();
    }

    let tuple =
        pgx_pg_sys::BuildTupleFromCStrings(attinmeta, values as *mut *mut ::std::os::raw::c_char);

    pgx_pg_sys::ExecStoreHeapTuple(tuple, slot, false);

    (*state).rownum += 1;

    slot
}

unsafe extern "C" fn hello_re_scan_foreign_scan(node: *mut pgx_pg_sys::ForeignScanState) {
    debug1!("HelloFdw: hello_re_scan_foreign_scan");

    let state = (*node).fdw_state as *mut HelloFdwState;
    (*state).rownum = 0;
}

extern "C" fn hello_end_foreign_scan(_node: *mut pgx_pg_sys::ForeignScanState) {
    debug1!("HelloFdw: hello_end_foreign_scan");
}

extern "C" fn hello_analyze_foreign_table(
    _relation: pgx_pg_sys::Relation,
    _func: *mut pgx_pg_sys::AcquireSampleRowsFunc,
    totalpages: *mut pgx_pg_sys::BlockNumber,
) -> bool {
    debug1!("HelloFdw: hello_analyze_foreign_table");
    unsafe {
        *totalpages = 1;
    }
    true
}

//
// C call stubs (missing from pgx for pg13)
//
extern "C" {
    fn create_foreignscan_path(
        root: *mut pgx_pg_sys::PlannerInfo,
        rel: *mut pgx_pg_sys::RelOptInfo,
        target: *mut pgx_pg_sys::PathTarget,
        rows: f64,
        startup_cost: pgx_pg_sys::Cost,
        total_cost: pgx_pg_sys::Cost,
        pathkeys: *mut pgx_pg_sys::List,
        required_outer: pgx_pg_sys::Relids,
        fdw_outerpath: *mut pgx_pg_sys::Path,
        fdw_private: *mut pgx_pg_sys::List,
    ) -> *mut pgx_pg_sys::Path;
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_select_star() {
        Spi::run("CREATE FOREIGN DATA WRAPPER hello_fdw HANDLER hello_fdw_handler VALIDATOR hello_fdw_validator");
        Spi::run("CREATE SERVER hello_server FOREIGN DATA WRAPPER hello_fdw");
        Spi::run("CREATE FOREIGN TABLE hello_fdw_table (id text, data text) SERVER hello_server");

        let row = Spi::get_two::<String, String>("SELECT * FROM hello_fdw_table");

        assert_eq!(
            row,
            (
                Some("Hello,World".to_string()),
                Some("Hello,World".to_string())
            )
        );
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
