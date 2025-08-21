; highlights.scm

; Comments and Docstrings
(comment) @comment
(docstring) @comment.documentation

; ***************
; Keywords
; ***************
[
  "enum"
  "component"
] @keyword

; ***************
; Types and Identifiers
; ***************
(classname) @type
(propname) @property
(identifier) @variable

; Property declaration: highlight both name and type
(property_decl
  (propname) @property
  (classname) @type)

; Component usage: highlight the type (classname)
(component
  (classname) @type)

; Enum declaration: classname is a type
(enum_decl
  (classname) @type)

; ***************
; Values
; ***************
(boolean) @boolean
(string) @string
(pixels) @number
(fraction) @number
(percentage) @number

