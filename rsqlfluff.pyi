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

def lex(
    raw: Union[str, TemplatedFile], indent_template: bool, dialect: str
) -> Tuple[List[Token], List[Any]]: ...
