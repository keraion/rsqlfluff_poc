import time
import glob
from typing import Union

from sqlfluff.core.dialects import dialect_readout
from sqlfluff.core.parser.lexer import Lexer
from sqlfluff.core.templaters import TemplatedFile
import rsqlfluff


def run_lexer(dialect: str, sql: Union[str, TemplatedFile, rsqlfluff.TemplatedFile], file: str):
    lexer = Lexer(dialect=dialect)
    py_start = time.time()
    py_out = lexer.lex(sql)
    py_end = time.time()

    rs_start = time.time()
    rs_out = rsqlfluff.lex(sql, True, dialect)
    rs_end = time.time()
    for p, r in zip(py_out[0], rs_out[0]):
        assert p.raw == r.raw
    py_dur = py_end - py_start
    rs_dur = rs_end - rs_start
    # print(py_dur)
    # print(rs_dur)
    # print(((py_dur) - (rs_dur)) / (py_dur))
    # print(f"{dialect} Speedup: {(py_dur) / (rs_dur)}")
    if ((py_end - py_start) / (rs_end - rs_start)) < 1.0:
        sql_len = len(sql) if isinstance(sql, str) else len(sql.source_str)
        print(f"{dialect},{file},{sql_len},{py_dur},{rs_dur},{(py_dur) / (rs_dur)}")
    for i, (p, r) in enumerate(zip(py_out[0], rs_out[0])):
        if p.raw != r.raw:
            print(f"{file}: {p.raw} != {r.raw}")
            break


# Run large files from python string
for dialect in dialect_readout():
    sql_all = ""
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql_all += fh.read()
    run_lexer(dialect.label, sql_all, "all")

# Run large files from python TemplatedFile
for dialect in dialect_readout():
    sql_all = ""
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql_all += fh.read()

    tf = TemplatedFile.from_string(sql_all)
    run_lexer(dialect.label, tf, "all")

# Run for all the individual files as TemplatedFile
for dialect in dialect_readout():
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql = fh.read()
        run_lexer(dialect.label, sql, file)

# Run for all the individual files as TemplatedFile
for dialect in dialect_readout():
    for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in file:
            continue
        with open(file, "r", encoding="utf8") as fh:
            sql = fh.read()
        tf = TemplatedFile.from_string(sql)
        run_lexer(dialect.label, tf, file)

# TODO: fix this to run
# # Run for all the individual files as rust TemplatedFile
# for dialect in dialect_readout():
#     for file in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
#         if "obevo" in file:
#             continue
#         with open(file, "r", encoding="utf8") as fh:
#             sql = fh.read()
#         tf = rsqlfluff.TemplatedFile.from_string(sql)
#         run_lexer(dialect.label, tf, file)
