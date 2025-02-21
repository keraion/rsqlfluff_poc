import time

from sqlfluff.core.parser.lexer import Lexer
import rsqlfluff as rsql

sql = """CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        name TEXT,
        email TEXT UNIQUE
    );

    CREATE TABLE orders (
        id INTEGER PRIMARY KEY,
        user_id INTEGER,
        total_price DECIMAL(10,2),
        FOREIGN KEY(user_id) REFERENCES users(id)
    );

    INSERT INTO users (id, name, email) VALUES (1, 'John Doe', 'john@example.com');           
    INSERT INTO users (id, name, email) VALUES (2, 'Jane Doe', 'jane@example.com');
    INSERT INTO orders (id, user_id, total_price) VALUES (1, 1, 99.99);
    INSERT INTO orders (id, user_id, total_price) VALUES (2, 2, 149.49);
    
    SELECT * FROM users;
    SELECT * FROM orders WHERE total_price > 100;
    
    UPDATE users SET email = 'john.doe@example.com' WHERE id = 1;
    UPDATE orders SET total_price = 89.99 WHERE id = 1;
    
    DELETE FROM orders WHERE total_price < 100;
    DELETE FROM users WHERE id = 2;
"""

start = time.time()
print(rsql.lex(sql, True, "ansi"))
end = time.time()
print(end - start)


lexer = Lexer(dialect="ansi")
start = time.time()
print(lexer.lex(sql))
end = time.time()
print(end - start)

sql2 = """SELECT amount+1 AS 'amount' FROM num1;

SELECT höhe+1 AS 'höhe' FROM num1;


SELECT amount*2 AS 'amount' FROM num1;

SELECT höhe*2 AS 'höhe' FROM num1;


SELECT employees.personal.name, neighbors.area FROM neighbors, employees
WHERE employees.personal.address.zipcode=neighbors.area.zipcode AND neighbors.num_neighbors > 1;

SELECT mitarbeiter.persönlicher.name, nachbarn.bereich FROM nachbarn, mitarbeiter
WHERE mitarbeiter.persönlicher.adresse.zipcode=nachbarn.gebiet.zipcode AND nachbarn.nummer_nachbarn > 1;


SELECT itemkey AS key, IMPLODE(itemprice) WITHIN GROUP (ORDER BY itemprice) AS prices
    FROM filtered GROUP BY itemkey ORDER BY itemkey;

SELECT ключтовара AS key, IMPLODE(ценатовара) WITHIN GROUP (ORDER BY ценатовара) AS цены
    FROM отфильтровано GROUP BY ключтовара ORDER BY ключтовара;


SELECT State, APPROXIMATE_PERCENTILE(sales USING PARAMETERS percentiles='0.5') AS median
FROM allsales GROUP BY state;

SELECT Χώρα, APPROXIMATE_PERCENTILE(πωλήσεις USING PARAMETERS percentiles='0.5') AS διάμεσος
FROM όλεςτιςπωλήσεις GROUP BY χώρα;


SELECT customer_state, customer_key, annual_income, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY customer_state) AS PERCENTILE_CONT
   FROM customer_dimension WHERE customer_state IN ('DC','WI') ORDER BY customer_state, customer_key;

SELECT état_du_client, clé_client, revenu_annuel, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY revenu_annuel)
      OVER (PARTITION BY état_du_client) AS PERCENTILE_CONT
   FROM dimension_client WHERE état_du_client IN ('Provence','Сhampagne') ORDER BY état_du_client, clé_client;


SELECT customer_state, customer_key, annual_income,
      PERCENTILE_DISC(.2) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY customer_state) AS PERCENTILE_DISC
   FROM customer_dimension
   WHERE customer_state IN ('DC','WI')
   AND customer_key < 300
   ORDER BY customer_state, customer_key;

SELECT état_du_client, clé_client, revenu_annuel,
      PERCENTILE_DISC(.2) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY état_du_client) AS PERCENTILE_DISC
   FROM dimension_client
   WHERE état_du_client IN ('Provence','Сhampagne')
   AND clé_client < 300
   ORDER BY état_du_client, clé_client;

SELECT customer_state, customer_key, annual_income, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY annual_income)
      OVER (PARTITION BY customer_state) AS PERCENTILE_CONT
   FROM customer_dimension WHERE customer_state IN ('DC','WI') AND customer_key < 300
   ORDER BY customer_state, customer_key;

SELECT état_du_client, clé_client, revenu_annuel, PERCENTILE_CONT(0.5) WITHIN GROUP(ORDER BY revenu_annuel)
      OVER (PARTITION BY état_du_client) AS PERCENTILE_CONT
   FROM dimension_client WHERE état_du_client IN ('Provence','Сhampagne') AND clé_client < 300
   ORDER BY état_du_client, clé_client;


SELECT store_region, store_city||', '||store_state location, store_name, number_of_employees FROM store.store_dimension
     LIMIT 2 OVER (PARTITION BY store_region ORDER BY number_of_employees ASC);

SELECT регион_магазина, город_магазина||', '||область_магазина местоположение, имя_магазина, количество_сотрудников FROM магазины.измерение_магазины
     LIMIT 2 OVER (PARTITION BY регион_магазина ORDER BY количество_сотрудников ASC);


SELECT PREDICT_LINEAR_REG(waiting USING PARAMETERS model_name='myLinearRegModel') FROM
faithful ORDER BY id;

SELECT PREDICT_LINEAR_REG(attente USING PARAMETERS model_name='monRegModèleLinéaire') FROM
fidèle ORDER BY id;


SELECT INFER_EXTERNAL_TABLE_DDL('/data/people/*.parquet'
        USING PARAMETERS format = 'parquet', table_name = 'employees');

SELECT INFER_EXTERNAL_TABLE_DDL('/data/άνθρωποι/*.parquet'
        USING PARAMETERS format = 'parquet', table_name = 'εργαζόμενοι');


SELECT PREDICT_ARIMA(temperature USING PARAMETERS model_name='arima_temp', start=100, npredictions=10) OVER(ORDER BY time) FROM temp_data;

SELECT PREDICT_ARIMA(температура USING PARAMETERS model_name='arima_temp', start=100, npredictions=10) OVER(ORDER BY time) FROM временные_данные;

SELECT INFER_TABLE_DDL ('/data/*.json'
    USING PARAMETERS table_name='restaurants', format='json',
max_files=3, max_candidates=3);

SELECT INFER_TABLE_DDL ('/data/*.json'
    USING PARAMETERS table_name='εστιατόρια', format='json',
max_files=3, max_candidates=3);


SELECT PURGE_TABLE('store.store_sales_fact');

SELECT PURGE_TABLE('المتجر.متجر_مبيعات_المتجر');


SELECT MSE(obs, prediction) OVER()
   FROM (SELECT eruptions AS obs,
                PREDICT_LINEAR_REG (waiting USING PARAMETERS model_name='myLinearRegModel') AS prediction
         FROM faithful_testing) AS PredictionOutput;

SELECT MSE(наблюдения, предсказания) OVER()
   FROM (SELECT извержения AS наблюдения,
                PREDICT_LINEAR_REG (ожидания USING PARAMETERS model_name='myLinearRegModel') AS прогноз
         FROM верное_испытание) AS РезультатПрогноза;


SELECT ps[0] as q0, ps[1] as q1, ps[2] as q2, ps[3] as q3, ps[4] as q4
FROM (SELECT APPROXIMATE_PERCENTILE(sales USING PARAMETERS percentiles='0, 0.25, 0.5, 0.75, 1')
AS ps FROM allsales GROUP BY state) as s1;

SELECT pz[0] as q0, pz[1] as q1, pz[2] as q2, pz[3] as q3, pz[4] as q4
FROM (SELECT APPROXIMATE_PERCENTILE(Verkäufe USING PARAMETERS percentiles='0, 0.25, 0.5, 0.75, 1')
AS pz FROM alleVerkäufe GROUP BY Staat) as s1;


SELECT id.name, major, GPA FROM students
   WHERE id = ROW('alice',119, ARRAY['alice@example.com','ap16@cs.example.edu']);

SELECT ид.имя, курс, СРБАЛЛ FROM студенты
   WHERE ид = ROW('алиса',119, ARRAY['alice@example.com','ap16@cs.example.edu']);


SELECT E'first part o'
    'f a long line';

SELECT E'πρώτο μέρος μι'
    'ας μακράς γραμμής';


SELECT STRING_TO_ARRAY(name USING PARAMETERS collection_delimiter=' ') FROM employee;

SELECT STRING_TO_ARRAY(имя USING PARAMETERS collection_delimiter=' ') FROM сотрудники;

-- ALTER SCHEMA block
ALTER SCHEMA ms OWNER TO dbadmin CASCADE;

ALTER SCHEMA επιμελητεία OWNER TO διαχειριστής CASCADE;

ALTER SCHEMA логистика OWNER TO алиса CASCADE;

ALTER SCHEMA s1, s2 RENAME TO s3, s4;

ALTER SCHEMA εμπορικός, s2 RENAME TO продажи, s4;

-- ALTER TABLE block
ALTER TABLE public.store_orders ADD COLUMN expected_ship_date date;

ALTER TABLE public.κατάστημα_παραγγελίες ADD COLUMN αναμενόμενη_ημερομηνία_αποστολής date;

ALTER TABLE public.заказы_магазина ADD COLUMN ожиддаемая_дата_отгрузки date;

ALTER TABLE t33 OWNER TO Alice;

ALTER TABLE επιμελητεία OWNER TO διαχειριστής;

ALTER TABLE заказы OWNER TO алиса;

-- ARRAY block
SELECT (ARRAY['مسؤل', 'διαχειριστής', 'логистика', 'd', 'e'])[1];

-- Cast w/ whitespace
SELECT amount_of_honey :: FLOAT
FROM bear_inventory;

SELECT ποσότητα_μελιού :: FLOAT
FROM αρκούδα_αποθήκη;

SELECT количество_мёда :: FLOAT
FROM медвежий_склад;

-- COMMENT ON block
COMMENT ON AGGREGATE FUNCTION APPROXIMATE_MEDIAN(x FLOAT) IS 'alias of APPROXIMATE_PERCENTILE with 0.5 as its parameter';
COMMENT ON AGGREGATE FUNCTION APPROXIMATE_MEDIAN(x FLOAT) IS 'ψευδώνυμο APPROXIMATE_PERCENTILE με 0,5 ως παράμετρό του';
COMMENT ON AGGREGATE FUNCTION APPROXIMATE_MEDIAN(x FLOAT) IS 'псевдоним APPROXIMATE_PERCENTILE с 0,5 в качестве параметра';

COMMENT ON SCHEMA public  IS 'All users can access this schema';
COMMENT ON SCHEMA public  IS 'Όλοι οι χρήστες έχουν πρόσβαση σε αυτό το σχήμα';
COMMENT ON SCHEMA public  IS 'Все пользователи могут получить доступ к этой схеме';

-- COPY block
COPY public.customer_dimension (
    customer_since FORMAT 'YYYY'
)
   FROM STDIN
   DELIMITER ','
   NULL AS 'null'
   ENCLOSED BY '"';

COPY παραγγελίες.παραγγελίες_ανά_ημέρα (
    πελάτη_αφού FORMAT 'YYYY'
)
   FROM STDIN
   DELIMITER ','
   NULL AS 'null'
   ENCLOSED BY '"';

COPY заказы.заказы_на_день (
    клиент_с_даты FORMAT 'YYYY'
)
   FROM STDIN
   DELIMITER ','
   NULL AS 'null'
   ENCLOSED BY '"';

-- CREATE PROJECTION block
CREATE PROJECTION public.employee_dimension_super
    AS SELECT * FROM public.employee_dimension
    ORDER BY employee_key
    SEGMENTED BY hash(employee_key) ALL NODES;

CREATE PROJECTION εμπορικός.παραγγελίες_ανά_ημέρα
    AS SELECT * FROM εμπορικός.παραγγελίες
    ORDER BY employee_key
    SEGMENTED BY hash(employee_key) ALL NODES;

CREATE PROJECTION продажи.продажи_на_по_клиенту
    AS SELECT * FROM продажи.продажи_на_сегодня
    ORDER BY клиент
    SEGMENTED BY hash(клиент) ALL NODES;

-- CREATE SCHEMA block
CREATE SCHEMA s3 DEFAULT INCLUDE SCHEMA PRIVILEGES;
CREATE SCHEMA εμπορικός DEFAULT INCLUDE SCHEMA PRIVILEGES;
CREATE SCHEMA продажи DEFAULT INCLUDE SCHEMA PRIVILEGES;

-- unqouted identifiers
SELECT * FROM логистика.εμπορικός;

SELECT * FROM логистика.εμπορικός1;
SELECT * FROM логистика.εμπορικός_;
SELECT * FROM логистика.s$ales$;
SELECT * FROM логистика._εμπορικός;
SELECT * FROM логистика._1234εμπορικός;

SELECT * FROM логистика1.εμπορικός;
SELECT * FROM логистика_.εμπορικός;
SELECT * FROM p$ublic$.εμπορικός;
SELECT * FROM _логистика.εμπορικός;
SELECT * FROM _1234логистика.εμπορικός;

SELECT * FROM логистика1.εμπορικός1;
SELECT * FROM логистика1_.εμπορικός1_;
SELECT * FROM p$ublic1_$.s$ales1_$;

-- quoted identifiers
SELECT * FROM "12логистика"."12344εμπορικός";
SELECT * FROM "_1234логистика"."_1234εμπορικός";"""

start = time.time()
rs_out = rsql.lex(sql2, True, "vertica")
end = time.time()
print(end - start)


lexer = Lexer(dialect="vertica")
start = time.time()
py_out = lexer.lex(sql2)
end = time.time()
print(end - start)

sql3 = """-- Test qualifying datatype with schema
CREATE TABLE counters (
    my_type public.MY_TYPE
);

--CREATE TABLE films (
--    code        char(5) CONSTRAINT firstkey PRIMARY KEY,
--    title       varchar(40) NOT NULL,
--    did         integer NOT NULL,
--    date_prod   date,
--    kind        varchar(10),
--    len         interval hour to minute
--);

CREATE TABLE distributors (
     did    integer PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY,
     name   varchar(40) NOT NULL CHECK (name <> '')
);

CREATE TABLE array_int (
   vector  int[][]
);

--CREATE TABLE films (
--    code        char(5),
--    title       varchar(40),
--    did         integer,
--    date_prod   date,
--    kind        varchar(10),
--    len         interval hour to minute,
--    CONSTRAINT production UNIQUE(date_prod)
--);

CREATE TABLE distributors (
    did     integer CHECK (did > 100),
    name    varchar(40),
    long_varying char varying(100)
);

CREATE TABLE distributors (
    did     integer,
    name    varchar(40),
    CONSTRAINT con1 CHECK (did > 100 AND name <> '')
);

--CREATE TABLE films (
--    code        char(5),
--    title       varchar(40),
--    did         integer,
--    date_prod   date,
--    kind        varchar(10),
--    len         interval hour to minute,
--    CONSTRAINT code_title PRIMARY KEY(code,title)
--);

CREATE TABLE distributors (
    did     integer,
    name    varchar(40),
    PRIMARY KEY(did)
);

CREATE TABLE distributors (
    did     integer PRIMARY KEY,
    name    varchar(40)
);

CREATE TABLE distributors (
    name      varchar(40) DEFAULT 'Luso Films',
    did       integer DEFAULT nextval('distributors_serial'),
    modtime   timestamp DEFAULT current_timestamp
);

CREATE TABLE distributors (
    did     integer CONSTRAINT no_null NOT NULL,
    name    varchar(40) NOT NULL
);

CREATE TABLE distributors (
    did     integer,
    name    varchar(40) UNIQUE
);

CREATE TABLE distributors (
    did     integer,
    name    varchar(40),
    UNIQUE(name)
);

CREATE TABLE distributors (
    did     integer,
    name    varchar(40),
    UNIQUE(name) WITH (fillfactor=70)
)
WITH (fillfactor=70);

--CREATE TABLE circles (
--    c circle,
--    EXCLUDE USING gist (c WITH &&)
--);

CREATE TABLE cinemas (
        id serial,
        name text,
        location text
) TABLESPACE diskvol1;

CREATE TYPE employee_type AS (name text, salary numeric);

CREATE TABLE employees OF employee_type (
    PRIMARY KEY (name),
    salary WITH OPTIONS DEFAULT 1000
);

CREATE TABLE measurement (
    logdate         date not null,
    peaktemp        int,
    unitsales       int
) PARTITION BY RANGE (logdate);

CREATE TABLE measurement_year_month (
    logdate         date not null,
    peaktemp        int,
    unitsales       int
) PARTITION BY RANGE (EXTRACT(YEAR FROM logdate), EXTRACT(MONTH FROM logdate));

CREATE TABLE cities (
    city_id      bigserial not null,
    name         text not null,
    population   bigint
) PARTITION BY LIST (left(lower(name), 1));

CREATE TABLE orders (
    order_id     bigint not null,
    cust_id      bigint not null,
    status       text
) PARTITION BY HASH (order_id);

CREATE TABLE measurement_y2016m07
    PARTITION OF measurement (
    unitsales DEFAULT 0
) FOR VALUES FROM ('2016-07-01') TO ('2016-08-01');

CREATE TABLE measurement_ym_older
    PARTITION OF measurement_year_month
    FOR VALUES FROM (MINVALUE, MINVALUE) TO (2016, 11);

CREATE TABLE measurement_ym_y2016m11
    PARTITION OF measurement_year_month
    FOR VALUES FROM (2016, 11) TO (2016, 12);

CREATE TABLE measurement_ym_y2016m12
    PARTITION OF measurement_year_month
    FOR VALUES FROM (2016, 12) TO (2017, 01);

CREATE TABLE measurement_ym_y2017m01
    PARTITION OF measurement_year_month
    FOR VALUES FROM (2017, 01) TO (2017, 02);

CREATE TABLE cities_ab
    PARTITION OF cities (
    CONSTRAINT city_id_nonzero CHECK (city_id != 0)
) FOR VALUES IN ('a', 'b');

CREATE TABLE cities_ab
    PARTITION OF cities (
    CONSTRAINT city_id_nonzero CHECK (city_id != 0)
) FOR VALUES IN ('a', 'b') PARTITION BY RANGE (population);

CREATE TABLE cities_ab_10000_to_100000
    PARTITION OF cities_ab FOR VALUES FROM (10000) TO (100000);

CREATE TABLE orders_p1 PARTITION OF orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 0);

CREATE TABLE orders_p2 PARTITION OF orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 1);

CREATE TABLE orders_p3 PARTITION OF orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 2);

CREATE TABLE orders_p4 PARTITION OF orders
    FOR VALUES WITH (MODULUS 4, REMAINDER 3);

CREATE TABLE cities_partdef
    PARTITION OF cities DEFAULT;

CREATE UNLOGGED TABLE staging (
    event_type INTEGER
    , event_time TIMESTAMP
    , user_email VARCHAR
    , phone_number VARCHAR
    , processing_date DATE
    , PRIMARY KEY (event_type, event_time, user_email, phone_number, processing_date)
);

CREATE TABLE measurement (
city_id int NOT NULL,
logdate date NOT NULL,
peaktemp int,
unitsales int
) PARTITION BY RANGE (logdate);

CREATE TABLE public.public (
id serial NOT NULL,
name text NOT NULL,
group_name text NULL,
cluster_id int8 NULL,
date_created timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
date_updated timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
operation_id int4 NOT NULL DEFAULT '-1'::integer
);

CREATE TABLE main.test_table (
    "col1" character varying(40) NOT NULL,
    "col2" double precision
);

CREATE TABLE groups (
    group_id INTEGER PRIMARY KEY generated BY DEFAULT AS IDENTITY
);

CREATE TABLE users (
    user_id INTEGER PRIMARY KEY generated BY DEFAULT AS IDENTITY,
    group_id INTEGER REFERENCES groups (group_id) ON DELETE CASCADE,
    domain_id INTEGER REFERENCES groups (group_id) ON UPDATE RESTRICT,
    other_id INTEGER REFERENCES groups (group_id) MATCH SIMPLE
);

CREATE TABLE orders (
    id bigint NOT NULL DEFAULT NEXTVAL('orders_id_seq'::regclass),
    constraint_collate_constraints text UNIQUE COLLATE numeric NOT NULL PRIMARY KEY,
    constraints_collate text NOT NULL UNIQUE COLLATE numeric,
    collate_constraints text COLLATE numeric NOT NULL UNIQUE,
    nulls_distinct text UNIQUE NULLS DISTINCT,
    nulls_not_distinct text UNIQUE NULLS NOT DISTINCT,
    everything text UNIQUE NULLS DISTINCT WITH (arg1=3, arg5='str')
        USING INDEX TABLESPACE tblspace COLLATE numeric
);

CREATE TABLE primary_key_options (
    everything int PRIMARY KEY WITH (arg1=3, arg5='str') USING INDEX TABLESPACE tblspace NOT NULL
);


-- Use non-reserved `usage` word as a table identifier
CREATE TABLE IF NOT EXISTS quotas.usage(foo int);

-- Use non-reserved `usage` word as a column identifier
CREATE TABLE IF NOT EXISTS quotas.my_table(usage int);

-- NOT NULL both before and after a default constraint
CREATE TABLE with_constraints1 (
    col_1 boolean NOT NULL DEFAULT false
);
CREATE TABLE with_constraints2 (
    col_1 boolean DEFAULT false NOT NULL
);

-- default constraint expression
CREATE TABLE with_constraints3 (
    col_1 int DEFAULT (1 + 2) * (3 + 4) NOT NULL
);
CREATE TABLE with_constraints33 (
    col_1 int DEFAULT 1 + 2 * 3 + 4 NOT NULL
);
CREATE TABLE with_constraints4 (
    col_1 int DEFAULT (1 + 2 * 3 + 4) NOT NULL
);
CREATE TABLE with_constraints5 (
    col_1 bool DEFAULT (1 NOT IN (3, 4)) NOT NULL
);
CREATE TABLE with_constraints6 (
    col_1 bool NOT NULL DEFAULT (5 NOT IN (5, 6))
);

CREATE TABLE test_with_storage_param (
    col_1 boolean
) WITH (autovacuum_enabled=true);


CREATE TABLE test_with_storage_params (
    col_1 boolean
) WITH (autovacuum_enabled=true, vacuum_truncate=false);

CREATE TABLE tbl (
    -- All forms of character data types listed at:
    -- https://www.postgresql.org/docs/current/datatype-character.html
    col_char_varying_unlimited character varying,
    col_char_varying_limited character varying(50),
    col_varchar_unlimited varchar,
    col_varchar_limited varchar(50),

    col_character_default character,
    col_character_specified character(50),
    col_char_default char,
    col_char_specified character(50),

    col_text text,

    -- some types you'll find in pg_catalog
    col_system_char "char", -- this is NOT the same as unquoted char
    col_name name
);

-- Test out EXCLUDE constraints, as well as other more advanced index parameters on constraints

-- from https://www.postgresql.org/docs/15/rangetypes.html: basic usage
CREATE TABLE reservation (
    during tsrange,
    EXCLUDE USING gist (during WITH &&)
);
CREATE TABLE room_reservation (
    room text,
    during tsrange,
    EXCLUDE USING gist (room WITH =, during WITH &&)
);

-- all the gnarly options: not every option is valid, but this will parse successfully on PG 15.
CREATE TABLE no_using (
    field text,
    EXCLUDE (field WITH =) NOT DEFERRABLE INITIALLY IMMEDIATE NO INHERIT
);
CREATE TABLE many_options (
    field text,
    EXCLUDE USING gist (
        one WITH =,
        nulls_opclass nulls WITH =,
        nulls_last NULLS LAST WITH =,
        two COLLATE "en-US" opclass
            (opt1, opt2=5, opt3='str', ns.opt4, ns.opt5=6, ns.opt6='str', opt7=ASC)
            ASC NULLS FIRST WITH =,
        (two + 5) WITH =,
        myfunc(a, b) WITH =,
        myfunc_opclass(a, b) fop (opt=1, foo=2) WITH =,
        only_opclass opclass WITH =,
        desc_order DESC WITH =
    ) INCLUDE (a, b) WITH (idx_num = 5, idx_str = 'idx_value', idx_kw=DESC)
        USING INDEX TABLESPACE tblspc
        WHERE (field != 'def')
        DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE example_table () INHERITS (parent_table);

CREATE TABLE IF NOT EXISTS table2(
  col1 int,
  col2 int NOT NULL,
  col3 int,

  FOREIGN KEY (col1, col2)
  REFERENCES table1 (col1, col2)
  ON DELETE SET NULL (col1)
);

CREATE TABLE IF NOT EXISTS table2(
  col1 int,
  col2 int NOT NULL,
  col3 int,

  FOREIGN KEY (col1, col2)
  REFERENCES table1 (col1, col2)
  ON DELETE SET DEFAULT (col1)
);
"""

start = time.time()
rs_out = rsql.lex(sql3, True, "postgres")
end = time.time()
print(end - start)


lexer = Lexer(dialect="postgres")
start = time.time()
py_out = lexer.lex(sql3)
end = time.time()
print(end - start)
