from typing import Any, List, Optional, Tuple, Union

class Slice: ...

class RawFileSlice:
    raw: str
    slice_type: str
    source_idx: int
    block_idx: int
    tag: Optional[str]

class TemplatedFileSlice:
    slice_type: str
    source_slice: Slice
    templated_slice: Slice

class TemplatedFile:
    source_str: str
    fname: str
    template_str: str
    sliced_file: List[TemplatedFileSlice]
    raw_sliced: List[RawFileSlice]

class Token:
    raw: str

class SQLLexError: ...

class Lexer:
    def __init__(self, dialect: str): ...
    def lex(
        self, lex_input: Union[str, TemplatedFile], template_blocks_indent: bool
    ) -> Tuple[List[Token], List[Any]]: ...
