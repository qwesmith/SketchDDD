" Vim indent file
" Language: SketchDDD
" Maintainer: SketchDDD Team

if exists("b:did_indent")
  finish
endif
let b:did_indent = 1

setlocal indentexpr=GetSketchDDDIndent()
setlocal indentkeys=0{,0},0),0],!^F,o,O

if exists("*GetSketchDDDIndent")
  finish
endif

function! GetSketchDDDIndent()
  let line = getline(v:lnum)
  let previousNum = prevnonblank(v:lnum - 1)
  let previous = getline(previousNum)

  " Default to same indent as previous line
  let ind = indent(previousNum)

  " Increase indent after opening braces
  if previous =~ '{\s*$'
    let ind += shiftwidth()
  endif

  " Decrease indent for closing braces
  if line =~ '^\s*}'
    let ind -= shiftwidth()
  endif

  return ind
endfunction
