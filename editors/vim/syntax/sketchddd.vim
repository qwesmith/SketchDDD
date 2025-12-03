" Vim syntax file
" Language: SketchDDD
" Maintainer: SketchDDD Team
" Latest Revision: 2024

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword sketchdddKeyword context entity value enum aggregate morphisms map pattern mappings root contains invariant
syn keyword sketchdddPattern CustomerSupplier AntiCorruptionLayer OpenHostService Conformist SharedKernel Partnership

" Primitive types
syn keyword sketchdddType String Int Float Bool UUID DateTime Date Decimal Email
syn keyword sketchdddGeneric List Map Set

" Comments
syn match sketchdddComment "//.*$"
syn region sketchdddComment start="/\*" end="\*/"

" Strings
syn region sketchdddString start='"' end='"' contains=sketchdddEscape
syn match sketchdddEscape "\\." contained

" Numbers
syn match sketchdddNumber "\<\d\+\>"
syn match sketchdddFloat "\<\d\+\.\d\+\>"

" Operators
syn match sketchdddOperator "->"
syn match sketchdddOperator "=>"
syn match sketchdddOperator "="
syn match sketchdddOperator "|"
syn match sketchdddOperator "?"
syn match sketchdddOperator "=="
syn match sketchdddOperator "!="
syn match sketchdddOperator "<="
syn match sketchdddOperator ">="
syn match sketchdddOperator "<"
syn match sketchdddOperator ">"
syn match sketchdddOperator "+"
syn match sketchdddOperator "-"
syn match sketchdddOperator "\*"
syn match sketchdddOperator "/"
syn match sketchdddOperator "%"
syn match sketchdddOperator "&&"
syn match sketchdddOperator "||"
syn match sketchdddOperator "!"

" Type names (PascalCase)
syn match sketchdddTypeName "\<[A-Z][a-zA-Z0-9]*\>"

" Field names (camelCase)
syn match sketchdddField "\<[a-z][a-zA-Z0-9_]*\s*:" contains=sketchdddColon
syn match sketchdddColon ":" contained

" Morphism names
syn match sketchdddMorphism "\<[a-z][a-zA-Z0-9]*\s*:\s*[A-Z]"me=e-2

" Annotations
syn match sketchdddAnnotation "@[a-zA-Z][a-zA-Z0-9]*\(([^)]*)\)\?"

" Enum variants
syn match sketchdddEnumVariant "\(|\s*\)\@<=[A-Z][a-zA-Z0-9]*"
syn match sketchdddEnumVariant "=\s*[A-Z][a-zA-Z0-9]*"ms=s+1

" Highlighting
hi def link sketchdddKeyword     Keyword
hi def link sketchdddPattern     Constant
hi def link sketchdddType        Type
hi def link sketchdddGeneric     Type
hi def link sketchdddComment     Comment
hi def link sketchdddString      String
hi def link sketchdddEscape      Special
hi def link sketchdddNumber      Number
hi def link sketchdddFloat       Float
hi def link sketchdddOperator    Operator
hi def link sketchdddTypeName    Type
hi def link sketchdddField       Identifier
hi def link sketchdddMorphism    Function
hi def link sketchdddAnnotation  PreProc
hi def link sketchdddEnumVariant Constant

let b:current_syntax = "sketchddd"
