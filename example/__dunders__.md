# Dunder methods

| Group                                 | methods           | notes                                                                      |
|---------------------------------------|-------------------|----------------------------------------------------------------------------|
| Object lifecycle                      | __new__           |                                                                            |
|                                       | __init__          |                                                                            |
|                                       | __del__           |                                                                            |
| Object representation                 | __repr__          |                                                                            |
|                                       | __str__           |                                                                            |
|                                       | __format__        |                                                                            |
|                                       | __bytes__         |                                                                            |
| Hashing / truth / identity            | __hash__          |                                                                            |
|                                       | __bool__          |                                                                            |
| Rich comparison                       | __eq__            |                                                                            |
|                                       | __ne__            |                                                                            |
|                                       | __lt__            |                                                                            |
|                                       | __le__            |                                                                            |
|                                       | __gt__            |                                                                            |
|                                       | __ge__            |                                                                            |
| Attribute access                      | __getattribute__  |                                                                            |
|                                       | __getattr__       |                                                                            |
|                                       | __setattr__       |                                                                            |
|                                       | __delattr__       |                                                                            |
|                                       | __dir__           |                                                                            |
| Attribute descriptors                 | __get__           |                                                                            |
|                                       | __set__           |                                                                            |
|                                       | __delete__        |                                                                            |
|                                       | __set_name__      |                                                                            |
| Class creation / metaclass hooks      | __prepare__       |                                                                            |
|                                       | __init_subclass__ |                                                                            |
|                                       | __class_getitem__ |                                                                            |
|                                       | __mro_entries__   |                                                                            |
| Instance / class checks               | __instancecheck__ |                                                                            |
|                                       | __subclasscheck__ |                                                                            |
| Numeric operators (binary)            | __add__           |                                                                            |
|                                       | __sub__           |                                                                            |
|                                       | __mul__           |                                                                            |
|                                       | __matmul__        |                                                                            |
|                                       | __truediv__       |                                                                            |
|                                       | __floordiv__      |                                                                            |
|                                       | __mod__           |                                                                            |
|                                       | __divmod__        |                                                                            |
|                                       | __pow__           |                                                                            |
|                                       | __lshift__        |                                                                            |
|                                       | __rshift__        |                                                                            |
|                                       | __and__           |                                                                            |
|                                       | __xor__           |                                                                            |
|                                       | __or__            |                                                                            |
| Reflected numeric operators           | __radd__          |                                                                            |
|                                       | __rsub__          |                                                                            |
|                                       | __rmul__          |                                                                            |
|                                       | __rmatmul__       |                                                                            |
|                                       | __rtruediv__      |                                                                            |
|                                       | __rfloordiv__     |                                                                            |
|                                       | __rmod__          |                                                                            |
|                                       | __rdivmod__       |                                                                            |
|                                       | __rpow__          |                                                                            |
|                                       | __rlshift__       |                                                                            |
|                                       | __rrshift__       |                                                                            |
|                                       | __rand__          |                                                                            |
|                                       | __rxor__          |                                                                            |
|                                       | __ror__           |                                                                            |
| In-place operators                    | __iadd__          |                                                                            |
|                                       | __isub__          |                                                                            |
|                                       | __imul__          |                                                                            |
|                                       | __imatmul__       |                                                                            |
|                                       | __itruediv__      |                                                                            |
|                                       | __ifloordiv__     |                                                                            |
|                                       | __imod__          |                                                                            |
|                                       | __ipow__          |                                                                            |
|                                       | __ilshift__       |                                                                            |
|                                       | __irshift__       |                                                                            |
|                                       | __iand__          |                                                                            |
|                                       | __ixor__          |                                                                            |
|                                       | __ior__           |                                                                            |
| Unary operators                       | __pos__           |                                                                            |
|                                       | __neg__           |                                                                            |
|                                       | __abs__           |                                                                            |
|                                       | __invert__        |                                                                            |
| Type conversion                       | __complex__       |                                                                            |
|                                       | __int__           |                                                                            |
|                                       | __float__         |                                                                            |
|                                       | __round__         |                                                                            |
|                                       | __index__         |                                                                            |
| Container protocol                    | __len__           |                                                                            |
|                                       | __length_hint__   |                                                                            |
|                                       | __iter__          |                                                                            |
|                                       | __next__          |                                                                            |
|                                       | __reversed__      |                                                                            |
|                                       | __contains__      |                                                                            |
|                                       | __getitem__       |                                                                            |
|                                       | __setitem__       |                                                                            |
|                                       | __delitem__       |                                                                            |
| Callable                              | __call__          |                                                                            |
| Context managers                      | __enter__         |                                                                            |
|                                       | __exit__          |                                                                            |
|                                       | __aenter__        |                                                                            |
|                                       | __aexit__         |                                                                            |
| Async protocol                        | __await__         |                                                                            |
|                                       | __aiter__         |                                                                            |
|                                       | __anext__         |                                                                            |
| Pickling / serialization hooks        | __getstate__      |                                                                            |
|                                       | __setstate__      |                                                                            |
|                                       | __reduce__        |                                                                            |
|                                       | __reduce_ex__     |                                                                            |
| Buffer protocol                       | __buffer__        | conceptual; actual CPython buffer API is C-level, not fully dunder-exposed |
| Async iteration / generator internals | __del__           | generator finalization context                                             |
|                                       | __await__         | already listed                                                             |
|                                       | __iter__          | generator reuse context                                                    |
| Misc / rarely used but real           | __fspath__        | filesystem path protocol                                                   |
|                                       | __class__         |                                                                            |
|                                       | __doc__           |                                                                            |
|                                       | __annotations__   |                                                                            |
