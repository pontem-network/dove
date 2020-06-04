//      BinOp = (listed from lowest to highest precedence)
//          "==>"                                       spec only
//          | "||"
//          | "&&"
//          | "==" | "!=" | "<" | ">" | "<=" | ">="
//          | ".."                                      spec only
//          | "|"
//          | "^"
//          | "&"
//          | "<<" | ">>"
//          | "+" | "-"l
//          | "*" | "/" | "%"
const PREC = {
  assign: 1,
  implies: 2, // ==>
  or: 3, // ||
  and: 4, // &&
  eq: 5, // ==
  neq: 5, // !=
  lt: 5, // <
  gt: 5, // >
  le: 5, // <=
  ge: 5, // >=
  range: 6, // ..
  bitor: 7, // |
  xor: 8, // ^
  bitand: 9, // &
  shl: 10, // <<
  shr: 10, // >>
  add: 11, // +
  sub: 11, // -
  mul: 12, // *
  div: 12, // /
  mod: 12, // %,
  unary: 13,
  field: 14,
  call: 15,
}

module.exports = grammar({
  name: 'move',
  extras: $ => [/\s/, $.line_comment],
  conflicts: $ => [
    [$._struct_identifier, $._variable_identifier, $._function_identifier],
    [$._struct_identifier, $._function_identifier],
    [$.function_type_params],
    [$.name_expression],
  ],

  rules: {
    source_file: $ => repeat($._statement),
    _statement: $ => choice(
      $.address_block,
      $.script_block,
      $.module_definition,

      // the following is additional to make move code pieces highlighting work.
      $._block_item,
      $.use_decl,
      $._function_definition,
      $._struct_definition,
    ),

    address_block: $ => seq(
      'address',
      field('address', $.address_literal),
      "{",
      repeat($.module_definition),
      "}"
    ),
    script_block: $ => seq(
      "script",
      "{",
      repeat($.use_decl),
      optional($.usual_function_definition),
      repeat($.spec_block),
      "}"
    ),
    // parse use declaration
    use_decl: $ => seq(
      'use',
      $._module_ident,
      optional(
        choice(
          $._use_alias, // use moudle
          seq('::', $.use_member), // use module member
          seq('::', '{', sepBy1(',', $.use_member), '}') // use multi module members
        )
      ),
      ';'
    ),
    use_member: $ => seq(
      field('member', $.identifier),
      optional($._use_alias),
    ),
    _use_alias: $ => seq(
      'as',
      field('as', $._module_identifier)
    ),


    module_definition: $ => {
      return seq(
        'module',
        field('name', $._module_identifier),
        field('body', $.module_body),
      );
    },
    module_body: $ => {
      return seq(
        '{',
        repeat(choice(
          $.use_decl,
          $._function_definition,
          $._struct_definition,
          $.spec_block,
        )),
        '}'
      );
    },

    _struct_definition: $ => choice(
      $.native_struct_definition,
      $.struct_definition,
    ),
    native_struct_definition: $ => seq(
      'native',
      $._struct_signature,
      ';'
    ),
    struct_definition: $ => seq(
      $._struct_signature,
      field('fields', $.struct_def_fields),
    ),
    struct_def_fields: $ => seq(
      '{',
      sepBy(',', $.field_annotation),
      '}'
    ),
    field_annotation: $ => seq(
      field('field', $._field_identifier),
      ':',
      field('type', $._type),
    ),

    _struct_signature: $ => seq(
      optional('resource'),
      'struct',
      field('name', $._struct_identifier),
      optional(field('type_parameters', $.type_parameters)),
    ),

    _function_definition: $ => choice(
      $.native_function_definition,
      $.usual_function_definition,
    ),
    native_function_definition: $ => seq(
      'native',
      $._function_signature,
      ';'
    ),
    usual_function_definition: $ => seq(
      $._function_signature,
      field('body', $.block)
    ),
    _function_signature: $ => seq(
      optional('public'),
      'fun',
      field('name', $._function_identifier),
      optional(field('type_parameters', $.type_parameters)),
      field('params', $.func_params),
      optional(seq(':', field('return_type', $._type))),
      optional(field('acquires', $.resource_accquires)),
    ),
    func_params: $ => seq(
      '(',
      sepBy(',', $.function_parameter),
      ')',
    ),
    resource_accquires: $ => seq(
      'acquires', sepBy1(',', $.module_access)
    ),

    //// Spec block start
    spec_block: $ => seq(
      'spec',
      optional(choice(
        seq('fun', $._function_identifier),
        seq('struct', $._struct_identifier),
        'module',
        seq(
          'schema',
          $._struct_identifier,
          optional(field('type_parameters', $.type_parameters)),
        ),
      )),
      field('body', $.spec_body)
    ),

    spec_body: $ => seq(
      '{',
      repeat($.use_decl),
      repeat($._spec_block_memeber),
      '}'
    ),
    _spec_block_memeber: $ => choice(
      $.spec_invariant,
      $._spec_function,
      $.spec_condition,
      $.spec_include,
      $.spec_apply,
      $.spec_pragma,
      $.spec_variable
    ),
    spec_condition: $ => seq(
      choice('assert', 'assume', 'decreases', 'aborts_if', 'ensures', 'requires'),
      $._expression,
      ';'
    ),
    spec_invariant: $ => seq(
      'invariant',
      optional(choice('update', 'pack', 'unpack')),
      $._expression,
      ';'
    ),
    spec_include: $ => seq('include', $._expression, ';'),

    spec_apply: $ => seq(
      'apply',
      field('exp', $._expression),
      'to',
      sepBy1(',', $.spec_apply_pattern),
      optional(seq('except', sepBy1(',', $.spec_apply_pattern))),
      ';'
    ),
    spec_apply_pattern: $ => seq(
      optional(choice('public', 'internal')),
      field('name_pattern', $.spec_apply_name_pattern),
      optional(field('type_parameters', $.type_parameters)),
    ),
    spec_apply_name_pattern: $ => /[A-Za-z0-9*]+/,

    spec_pragma: $ => seq(
      'pragma',
      sepBy(',', $.spec_pragma_property),
      ';'
    ),
    spec_pragma_property: $ => seq($.identifier, optional(seq('=', $._literal_value))),

    spec_variable: $ => seq(
      optional(choice('global', 'local')),
      field('name', $.identifier),
      optional(field('type_parameters', $.type_parameters)),
      ':',
      field('type', $._type),
      ';'
    ),

    _spec_function: $ => choice(
      $.native_spec_function,
      $.usual_spec_function
    ),

    native_spec_function: $ => seq(
      'native',
      'define',
      $._spec_function_signature,
      ';'
    ),
    usual_spec_function: $ => seq(
      'define',
      $._spec_function_signature,
      field('body', $.block)
    ),
    _spec_function_signature: $ => seq(
      field('name', $._function_identifier),
      optional(field('type_parameters', $.type_parameters)),
      field('params', $.func_params),
      seq(':', field('return_type', $._type)),
    ),

    //// Spec block end


    // move type grammar
    _type: $ => choice(
      // $.immutable_borrow_type,
      // $.mutable_borrow_type,
      $.apply_type,
      $.ref_type,
      $.tuple_type,
      // $.primative_type,
      // TODO: spec only
      $.function_type,
    ),
    apply_type: $ => seq(
      $.module_access,
      optional(field('type_arguments', $.type_arguments)),
    ),
    ref_type: $ => seq(
      choice('&', '&mut'),
      $._type
    ),
    tuple_type: $ => seq('(', sepBy1(',', $._type), ')'),
    primative_type: $ => choice(
      'u8',
      'u64',
      'u128',
      'bool',
      'address',
      'signer',
      'bytearray',
    ),
    module_access: $ => choice(
      $.identifier,
      seq('::', $.identifier),
      seq(field('module', $._module_identifier), '::', $.identifier),
      seq($._module_ident, '::', $.identifier)
    ),
    _module_ident: $ => seq(
      field('address', $.address_literal),
      '::',
      field('module', $._module_identifier)
    ),

    type_arguments: $ => seq(
      '<',
      sepBy1(',', $._type),
      '>'
    ),

    function_type: $ => seq(
      field('param_types', $.function_type_params),
      field('return_type', $._type)
    ),
    function_type_params: $ => seq('|', sepBy(',', $._type), '|'),

    // function parameter grammar
    function_parameter: $ => seq(
      field('name', $._variable_identifier),
      ':',
      field('type', $._type),
    ),

    // type parameter grammar
    type_parameters: $ => seq('<', sepBy1(',', $.type_parameter), '>'),
    type_parameter: $ => seq(
      $._type_parameter_identifier,
      optional(seq(':', choice('copyable', 'resource')))
    ),


    //// Block

    block: $ => seq(
      '{',
      repeat($._block_item),
      optional(choice($._expression, $.let_statement)),
      '}'
    ),
    _block_item: $ => seq(
      choice(
        $._expression,
        $.let_statement,
      ),
      ';'
    ),
    let_statement: $ => seq(
      'let',
      field('binds', $.bind_list),
      optional(seq(':', field('type', $._type))),
      optional(seq('=', field('exp', $._expression)))
    ),
    //// Block end


    //// Expression

    _expression: $ => choice(
      // TODO: spec only
      $.lambda_expression,
      $.if_expression,
      $.while_expression,
      $.loop_expression,
      $.return_expression,
      $.abort_expression,
      $.assign_expression,
      // unary expression is included in binary_op,
      $._unary_expression,
      $.binary_expression,
    ),

    lambda_expression: $ => seq(
      field('bindings', $.lambda_bindings),
      field('exp', $._expression)
    ),
    lambda_bindings: $ => seq(
      '|',
      sepBy(',', $._bind),
      '|'
    ),
    /// if-else expression
    if_expression: $ => prec.right(
      seq(
        'if',
        '(',
        field('eb', $._expression),
        ')',
        field('et', $._expression),
        optional(seq(
          'else',
          field('ef', $._expression)
        )),
      )
    ),

    //// while expression
    while_expression: $ => seq(
      'while',
      '(',
      field('eb', $._expression),
      ')',
      field('body', $._expression),
    ),

    //// loop expression
    loop_expression: $ => seq('loop', field('body', $._expression)),

    //// return expression
    return_expression: $ => seq('return', optional(field('return', $._expression))),
    // abort expression
    abort_expression: $ => seq('abort', field('abort', $._expression)),

    assign_expression: $ => prec.left(PREC.assign,
      seq(
        field('lhs', $._unary_expression),
        '=',
        field('rhs', $._expression)
      )
    ),

    //      BinOpExp =
    //          <BinOpExp> <BinOp> <BinOpExp>
    //          | <UnaryExp>
    _binary_operand: $ => choice(
      $._unary_expression,
      $.binary_expression,
    ),
    binary_expression: $ => {
      const table = [
        [PREC.implies, '==>'],
        [PREC.or, '||'],
        [PREC.and, '&&'],
        [PREC.eq, '=='],
        [PREC.neq, '!='],
        [PREC.lt, '<'],
        [PREC.gt, '>'],
        [PREC.le, '<='],
        [PREC.ge, '>='],
        [PREC.range, '..'],
        [PREC.bitor, '|'],
        [PREC.xor, '^'],
        [PREC.bitand, '&'],
        [PREC.shl, '<<'],
        [PREC.shr, '>>'],
        [PREC.add, '+'],
        [PREC.sub, '-'],
        [PREC.mul, '*'],
        [PREC.div, '/'],
        [PREC.mod, '%']
      ];

      let binary_expression = choice(...table.map(
        ([precedence, operator]) => prec.left(precedence, seq(
          field('lhs', $._binary_operand),
          field('operator', alias(operator, $.binary_operator)),
          field('rhs', $._binary_operand),
        ))
      ));

      return binary_expression;
    },

    _unary_expression: $ => choice(
      $.unary_expression,
      $.borrow_expression,
      $.dereference_expression,
      $.move_or_copy_expression,
      $._expression_term,
    ),
    unary_expression: $ => seq(
      field('op', $.unary_op),
      field('exp', $._unary_expression)
    ),
    unary_op: $ => choice('!'),

    // dereference
    dereference_expression: $ => prec(PREC.unary, seq(
      '*',
      field('exp', $._unary_expression),
    )),
    // borrow
    borrow_expression: $ => prec(PREC.unary, seq(
      choice('&', '&mut'),
      field('exp', $._unary_expression),
    )),
    // move or copy
    move_or_copy_expression: $ => prec(PREC.unary, seq(
      choice('move', 'copy'),
      field('exp', $._variable_identifier),
    )),

    // TODO: update module access parsing
    _expression_term: $ => choice(
      $.break_expression,
      $.continue_expression,
      $._name_exp,
      $._literal_value,
      $.unit_expression,
      $.expression_list,
      $.annotate_expression,
      $.cast_expression,
      $.block,
      $.spec_block,

      $.dot_expression,
      $.index_expression,
    ),
    break_expression: $ => choice('break'),
    continue_expression: $ => choice('continue'),
    _name_exp: $ => choice(
      $.name_expression,
      $.pack_expression,
      $.call_expression,
    ),
    name_expression: $ => prec(-1, seq(
      $.module_access,
      optional(field('type_arguments', $.type_arguments))
    )),
    pack_expression: $ => seq(
      $.name_expression,
      field('body', $.field_initialize_list),
    ),
    field_initialize_list: $ => seq(
      '{',
      sepBy(',', $.exp_field),
      '}'
    ),

    call_expression: $ => prec(PREC.call, seq(
      $.name_expression,
      field('args', $.arg_list),
    )),
    arg_list: $ => seq(
      '(',
      sepBy(',', $._expression),
      ')'
    ),

    expression_list: $ => seq('(', sepBy1(',', $._expression), ')'),
    unit_expression: $ => seq('(', ')'),
    cast_expression: $ => seq(
      '(',
      field('exp', $._expression),
      'as',
      field('ty', $._type),
      ')'
    ),
    annotate_expression: $ => seq(
      '(',
      field('exp', $._expression),
      ':',
      field('ty', $._type),
      ')'
    ),


    dot_expression: $ => prec.left(PREC.field, seq(
      field('e', $._expression_term),
      '.',
      field('f', $._field_identifier)
    )),
    index_expression: $ => prec.left(PREC.call, seq(
      field('e',
        $._expression_term,
      ),
      '[', field('idx', $._expression), ']'
    )),

    /// Expression end

    //// Fields and Bindings
    exp_field: $ => seq(
      field('field', $._field_identifier),
      optional(seq(
        ':',
        field('exp', $._expression)
      ))
    ),

    // The bindlist is enclosed in parenthesis, except that the parenthesis are
    // optional if there is a single Bind.
    bind_list: $ => choice(
      $._bind,
      seq('(', sepBy(',', $._bind), ')')
    ),
    _bind: $ => choice(
      alias($._variable_identifier, $.bind_var),
      $.bind_unpack,
    ),
    bind_unpack: $ => seq(
      $.module_access,
      optional(field('type_arguments', $.type_arguments)),
      field('bind_fields', $.bind_fields),
    ),
    bind_fields: $ => seq(
      '{', sepBy(',', $.bind_field), '}'
    ),
    bind_field: $ => seq(
      field('field', $._field_identifier), // direct bind
      optional(seq(
        ':',
        field('bind', $._bind)
      ))
    ),
    //// Fields and Bindings - End


    //// literal
    _literal_value: $ => choice(
      $.address_literal,
      $.bool_literal,
      $.num_literal,
      $.byte_string_literal,

    ),
    address_literal: $ => /0x[a-fA-F0-9]+/,
    bool_literal: $ => choice('true', 'false'),
    num_literal: $ => /[0-9][0-9]*(?:u8|u64|u128)?/,
    // TODO: tree-sitter not support ".*?"
    byte_string_literal: $ => /x"[0-9a-fA-F]*"/,


    _module_identifier: $ => alias($.identifier, $.module_identifier),
    _struct_identifier: $ => alias($.identifier, $.struct_identifier),
    // struct_type_name: $ => /[A-Z][0-9a-zA-Z_]*/, // must start with A-Z
    _function_identifier: $ => alias($.identifier, $.function_identifier),
    _variable_identifier: $ => alias($.identifier, $.variable_identifier),
    _field_identifier: $ => alias($.identifier, $.field_identifier),
    _type_parameter_identifier: $ => alias($.identifier, $.type_parameter_identifier),

    identifier: $ => /[a-zA-Z_][0-9a-zA-Z_]*/,

    line_comment: $ => token(seq(
      '//', /.*/
    )),

  }
});

//      (<rule> 'sep')* <rule>?
// Note that this allows an optional trailing `sep`.
function sepBy(sep, rule) {
  return seq(repeat(seq(rule, sep)), optional(rule));
}
function sepBy1(sep, rule) {
  return seq(rule, repeat(seq(sep, rule)), optional(sep));
}

