import time
import glob

from sqlfluff.core.dialects import dialect_readout
from sqlfluff.core.parser.lexer import Lexer
from sqlfluff.core.templaters import TemplatedFile
import rsqlfluff

for dialect in dialect_readout():
    print(dialect.label)
    sql_all = ""
    for f in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in f:
            continue
        with open(f, "r", encoding="utf8") as fh:
            sql_all += fh.read()

    lexer = Lexer(dialect=dialect.label)
    py_start = time.time()
    py_out = lexer.lex(sql_all)
    py_end = time.time()

    rs_start = time.time()
    rs_out = rsqlfluff.lex(sql_all, True, dialect.label)
    rs_end = time.time()
    for p, r in zip(py_out[0], rs_out[0]):
        assert p.raw == r.raw
    print(py_end - py_start)
    print(rs_end - rs_start)
    print(((py_end - py_start) - (rs_end - rs_start)) / (py_end - py_start))
    print(f"{dialect.label} Speedup: {(py_end - py_start) / (rs_end - rs_start)}")

for dialect in dialect_readout():
    for f in glob.glob(f"../sqlfluff/test/fixtures/dialects/{dialect.label}/*.sql"):
        if "obevo" in f:
            continue
        with open(f, "r", encoding="utf8") as fh:
            sql = fh.read()
        

        tf = TemplatedFile.from_string(sql)
        lexer = Lexer(dialect=dialect.label)
        py_start = time.time()
        py_out = lexer.lex(tf)
        py_end = time.time()

        # rtf = rsqlfluff.TemplatedFile.from_py_templated_file(tf)

        rs_start = time.time()
        rs_out = rsqlfluff.lex(tf, True, dialect.label)
        rs_end = time.time()
        for i, (p, r) in enumerate(zip(py_out[0], rs_out[0])):
            assert p.raw == r.raw
        if ((py_end - py_start) / (rs_end - rs_start)) < 1.0:
            print(f"{dialect.label}: {f}")
            print(py_end - py_start)
            print(rs_end - rs_start)
            print(((py_end - py_start) - (rs_end - rs_start)) / (py_end - py_start))
            print(f"{dialect.label} Speedup: {(py_end - py_start) / (rs_end - rs_start)}")
