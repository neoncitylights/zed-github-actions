; GitHub Actions-specific patterns (ordered before generic YAML patterns so they take priority)

; Structural keys used in workflow and composite action files
(block_mapping_pair
	key: (flow_node (plain_scalar (string_scalar) @keyword))
	(#match? @keyword "^(concurrency|container|continue-on-error|defaults|env|environment|id|if|inputs|jobs|matrix|name|needs|on|outputs|permissions|run|run-name|runs|runs-on|secrets|services|shell|steps|strategy|timeout-minutes|uses|with|working-directory)$"))

; `if:` values are implicitly expression context even without ${{ }}
(block_mapping_pair
	key: (flow_node (plain_scalar (string_scalar) @_key))
	value: (flow_node (plain_scalar (string_scalar) @string.special))
	(#eq? @_key "if"))

; Strings containing GitHub Actions expressions (${{ }})
([
	(double_quote_scalar)
	(single_quote_scalar)
	(string_scalar)
] @string.special
(#match? @string.special "\\$\\{\\{"))

; Generic YAML

(boolean_scalar) @boolean
(null_scalar) @constant.builtin

[
  (double_quote_scalar)
  (single_quote_scalar)
  (block_scalar)
  (string_scalar)
] @string

(escape_sequence) @string.escape

[
  (integer_scalar)
  (float_scalar)
] @number

(comment) @comment

[
  (anchor_name)
  (alias_name)
  (tag)
] @type

key: (flow_node (plain_scalar (string_scalar) @property))

[
 ","
 "-"
 ":"
 ">"
 "?"
 "|"
] @punctuation.delimiter

[
 "["
 "]"
 "{"
 "}"
] @punctuation.bracket

[
 "*"
 "&"
 "---"
 "..."
] @punctuation.special
