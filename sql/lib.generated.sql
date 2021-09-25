-- ./src/lib.rs:16:0
CREATE OR REPLACE FUNCTION "hello_fdw_validator"("_options" text[], "_oid" oid) RETURNS void STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'hello_fdw_validator_wrapper';
-- ./src/lib.rs:21:0
CREATE OR REPLACE FUNCTION "hello_fdw_handler"() RETURNS fdw_handler STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'hello_fdw_handler_wrapper';
CREATE SCHEMA IF NOT EXISTS "tests";
-- ./src/lib.rs:211:4
CREATE OR REPLACE FUNCTION tests."test_select_star"() RETURNS void LANGUAGE c AS 'MODULE_PATHNAME', 'test_select_star_wrapper';
