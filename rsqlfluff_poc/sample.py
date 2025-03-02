import time
import glob
from typing import Union

from sqlfluff.core.dialects import dialect_readout
from sqlfluff.core.parser.lexer import Lexer
from sqlfluff.core.templaters import TemplatedFile
import rsqlfluff

def run_lexer(
    name: str,
    dialect: str,
    sql: Union[str, TemplatedFile, rsqlfluff.TemplatedFile],
    file: str,
    print_results = True,
):
    py_start = time.time()
    lexer = Lexer(dialect=dialect)
    py_out = lexer.lex(sql)
    py_end = time.time()

    rs_start = time.time()
    lexer = rsqlfluff.Lexer(dialect=dialect)
    rs_out = lexer.lex(sql, True)
    rs_end = time.time()
    for p, r in zip(py_out[0], rs_out[0]):
        assert p.raw == r.raw
    py_dur = py_end - py_start
    rs_dur = rs_end - rs_start
    sql_len = len(sql) if isinstance(sql, str) else len(sql.source_str)
    if print_results:
        print(f"{name},{dialect},{file},{sql_len},{py_dur},{rs_dur},{(py_dur) / (rs_dur)}")
    for i, (p, r) in enumerate(zip(py_out[0], rs_out[0])):
        if p.raw != r.raw:
            print(f"{file}: {p.raw} != {r.raw}")
            break


# print("name,dialect,file,sql_len,py_dur,rs_dur,py_dur/rs_dur")

# Load all the dialects before measuring, don't print the output here
for dialect in dialect_readout():
    sql_all = ""
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql_all += fh.read()
        break
    run_lexer("all_py_str", dialect.label, sql_all, "all", False)

# Run large files from python string
for dialect in dialect_readout():
    sql_all = ""
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql_all += fh.read()
    run_lexer("all_py_str", dialect.label, sql_all, "all")

# Run large files from python TemplatedFile
for dialect in dialect_readout():
    sql_all = ""
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql_all += fh.read()

    tf = TemplatedFile.from_string(sql_all)
    run_lexer("all_py_tf", dialect.label, tf, "all")

# Run large files from rust TemplatedFile
for dialect in dialect_readout():
    sql_all = ""
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql_all += fh.read()

    tf = rsqlfluff.TemplatedFile.from_string(sql_all)
    run_lexer("all_rust_tf", dialect.label, tf, "all")

# Run for all the individual files as strings
for dialect in dialect_readout():
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql = fh.read()
        run_lexer("indiv_py_str", dialect.label, sql, file)

# Run for all the individual files as TemplatedFile
for dialect in dialect_readout():
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql = fh.read()
        tf = TemplatedFile.from_string(sql)
        run_lexer("indiv_py_tf", dialect.label, tf, file)


# Run for all the individual files as rust TemplatedFile
for dialect in dialect_readout():
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql = fh.read()
        tf = rsqlfluff.TemplatedFile.from_string(sql)
        run_lexer("indiv_rust_tf", dialect.label, tf, file)
