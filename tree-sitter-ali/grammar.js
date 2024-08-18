const
  PREC = {
    primary: 7,
    unary: 6,
    multiplicative: 5,
    additive: 4,
    comparative: 3,
    and: 2,
    or: 1,
  },

  type = choice('Action', 'Array', 'Automation', 'BigInteger', 'BigText', 'Blob', 'Boolean', 'Byte', 'Char', 'ClientType', 'Code', 'Codeunit', 'ConnectionType', 'Database', 'Dataclassification', 'DataScope', 'Date', 'DateFormula', 'DateTime', 'Decimal', 'DefaultLayout', 'Dialog', 'Dictionary', 'DotNet', 'DotNetAssembly', 'DotNetTypeDeclaration', 'Duration', 'Enum', 'ErrorInfo', 'ErrorType', 'ExecutionContext', 'ExecutionMode', 'FieldClass', 'FieldRef', 'FieldType', 'File', 'FilterPageBuilder', 'Guid', 'HttpClient', 'HttpContent', 'HttpHeaders', 'HttpRequestMessage', 'HttpResponseMessage', 'InStream', 'Integer', 'Joker', 'JsonArray', 'JsonObject', 'JsonToken', 'JsonValue', 'KeyRef', 'List', 'ModuleDependencyInfo', 'ModuleInfo', 'None', 'Notification', 'NotificationScope', 'ObjectType', 'Option', 'OutStream', 'Page', 'PageResult', 'Query', 'Record', 'RecordId', 'RecordRef', 'Report', 'ReportFormat', 'SecurityFilter', 'SecurityFiltering', 'SessionSettings', 'Table', 'TableConnectionType', 'TableFilter', 'TestAction', 'TestField', 'TestFilterField', 'TestPage', 'TestPermissions', 'TestRequestPage', 'Text', 'TextBuilder', 'TextConst', 'TextEncoding', 'Time', 'TransactionModel', 'TransactionType', 'Variant', 'Verbosity', 'Version', 'View', 'Views', 'WebServiceActionContext', 'WebServiceActionResultCode', 'XmlAttribute', 'XmlAttributeCollection', 'XmlCData', 'XmlComment', 'XmlDeclaration', 'XmlDocument', 'XmlDocumentType', 'XmlElement', 'XmlNamespaceManager', 'XmlNameTable', 'XmlNode', 'XmlNodeList', 'XmlPort', 'XmlProcessingInstruction', 'XmlReadOptions', 'XmlText', 'XmlWriteOptions', 'Label'),
  object_with_id_type = choice('codeunit', 'enum', 'enumextension', 'page', 'pageextension', 'permissionset', 'permissionsetextension', 'profile', 'query', 'report', 'reportextension', 'table', 'tableextension', 'xmlport'),
  object_without_id_type = choice('pagecustomization', 'dotnet', 'controladdin', 'interface', 'entitlement', 'assembly'),
  metadata_type = choice('action', 'actions', 'add', 'addafter', 'addbefore', 'addfirst', 'addlast', 'area', 'assembly', 'chartpart', 'column', 'cuegroup', 'customizes', 'dataitem', 'dataset', 'elements', 'extends', 'field', 'fieldattribute', 'fieldelement', 'fieldgroup', 'fieldgroups', 'fields', 'filter', 'fixed', 'grid', 'group', 'implements', 'key', 'keys', 'label', 'labels', 'layout', 'modify', 'moveafter', 'movebefore', 'movefirst', 'movelast', 'part', 'repeater', 'requestpage', 'schema', 'separator', 'systempart', 'tableelement', 'textattribute', 'textelement', 'type', 'value'),
  method_keyword = choice('trigger', 'procedure'),
  fn_like_property_keyword = choice('average', 'const', 'count', 'exist', 'field', 'filter', 'lookup', 'max', 'min', 'order', 'sorting', 'sum', 'upperlimit', 'where', 'ascending', 'descending'),
  property_keyword = choice('tabledata'),
  boolean_literal = choice('true', 'false'),
  decimal_literal = token(/\d+\.\d+/),
  integer_literal = token(/\d+/),
  number_literal = choice(decimal_literal, integer_literal),

  comparative_operators = ['==', '!=', '<', '<=', '>', '>=', 'in'],
  additive_operators = ['+', '-'],
  multiplicative_operators = ['*', '/', 'div', '%'],
  and_operator = 'and',
  or_operators = ['or', '^'],
  unary_operators = additive_operators.concat('not'),
  property_unary_expression_operators = unary_operators.concat(comparative_operators),
  assignment_operators = additive_operators.concat(multiplicative_operators).map(operator => operator + '=').concat('=');

module.exports = grammar({
  name: 'ali',

  extras: $ => [
    $.comment,
    $.preprocessor,
    /\s/,
  ],

  word: $ => $._normal_identifier,

  rules: {
    source_file: $ => seq(
      optional($.namespace_declaration),
      repeat($.using_directive),
      $.object_definition,
    ),
    namespace_declaration: _ => seq(
      'namespace',
      /[^;]*;/
    ),
    using_directive: _ => seq(
      'using',
      /[^;]*;/
    ),

    object_definition: $ => seq(
      choice(
        seq(
          object_with_id_type,
          $.number_literal
        ),
        object_without_id_type,
      ),
      $.identifier,
      optional(seq('extends', $.identifier)),
      $.metadata_block,
    ),

    _var_type: $ => choice($.assignable_var_type, $.non_assignable_var_type),
    assignable_var_type: $ => choice(
      $._type_with_length,
      type,
    ),
    non_assignable_var_type: $ => choice(
      $._object_with_name_type,
      $._type_with_properties,
      $._array_type,
      $._option_type,
    ),
    _type_with_length: _ => seq(type, /\[\d+(,\s*\d+)?\]/),
    _object_with_name_type: $ => seq(type, $.identifier),
    _type_with_properties: $ => seq(
      type,
      choice($.string_literal, seq($.identifier, '=', $.expression)),
      repeat(seq(',', $.identifier, '=', $.expression))
    ),
    _array_type: $ => seq(
      type,
      /\[\d+(,\s*\d+)?\]/,
      repeat1(seq('of', $.assignable_var_type))
    ),
    _option_type: $ => seq(type, $.identifier, repeat1(seq(',', $.identifier))),

    property_definition: $ => prec(1, seq(
      $.identifier,
      '=',
      $.property_value,
      $.statement_end,
    )),
    property_value: $ => prec(1, choice(
      $.decimal_places,
      $.property_value_list,
      $._property_value_recursive,
    )),
    _property_value_recursive: $ => alias(
      $._property_value_recursive_inner,
      $.property_value,
    ),
    _property_value_recursive_inner: $ => prec.right(1, choice(
        $._fn_like_property_value,
        $._property_filtered_identifier,
        seq($.property_keyword, $.identifier),
        $.property_expression,
        seq($._property_value_recursive, '=', $._property_value_recursive),
    )),
    property_value_list: $ => seq(
      $._property_value_recursive,
      repeat(seq(',', $._property_value_recursive)),
    ),
    property_keyword: _ => property_keyword,
    _fn_like_property_keyword: _ => fn_like_property_keyword,
    _fn_like_property_value: $ => seq(
      alias($._fn_like_property_keyword, $.property_keyword),
      '(',
      $.property_value_list,
      ')'
    ),
    _property_filtered_identifier: $ => seq($.identifier, $._fn_like_property_value),
    decimal_places: $ => choice(
      seq($._decimal_places_integer, ':', $._decimal_places_integer),
      seq(':', $._decimal_places_integer),
      seq($._decimal_places_integer, ':'),
    ),
    _decimal_places_integer: $ => alias(
      token(prec(1, integer_literal)),
      $.integer_literal
    ),

    property_expression: $ => prec(1, choice( // expression without method call
      $.property_unary_expression,
      $.property_binary_expression,
      $.property_parenthesized_expression,
      $.property_range_expression,
      $.identifier,
      $.number_literal,
      $.boolean_literal,
      $.string_literal,
      $.date_literal,
    )),

    metadata: $ => seq(
      $.metadata_type,
      optional($.metadata_arguments),
      $.metadata_block
    ),
    metadata_type: _ => metadata_type,
    metadata_arguments: $ => seq(
      '(',
      optional(choice($.identifier, $.expression)),
      repeat(seq($.statement_end, choice($.identifier, $.expression))),
      ')'
    ),
    metadata_block: $ => seq(
      $.metadata_begin,
      repeat($.property_definition),
      repeat($.metadata),
      repeat($.global_var_definitions),
      repeat($.method_definition),
      $.metadata_end,
    ),
    metadata_begin: _ => /\{/,
    metadata_end: _ => /\}/,

    global_var_definitions: $ => seq(
      optional('protected'),
      $.declaration_keyword,
      repeat1(seq(
        $.identifier,
        $.var_type_declaration,
        $.statement_end,
      )),
    ),

    method_definition: $ => seq(
      optional($.method_attributes),
      optional('local'),
      method_keyword,
      $.identifier,
      $.parameters,
      optional($.return_var),
      $.code_block,
    ),
    method_attributes: $ => repeat1(seq(
      '[',
      $.identifier,
      optional(seq(
        '(',
        repeat(choice(
          ',',
          $.expression,
        )),
        ')',
      )),
      ']',
    )),
    _attribute_expression: $ => $.expression,
    _attribute_identifier: $ => $.identifier,
    parameters: $ => seq(
      '(',
      optional($._var_definition),
      repeat(seq($.statement_end, $._var_definition)),
      ')'
    ),
    return_var: $ => seq(
      optional($.identifier),
      ':',
      /[^{]+/,
    ),
    _var_definition: $ => seq(
      $.identifier,
      ':',
      $._var_type,
    ),
    code_block: $ => seq(
      $.code_begin,
      repeat(seq($._code_statement)),
      $.code_end
    ),
    code_begin: _ => /\{/,
    code_end: _ => /\}/,

    _code_statement: $ => choice(
      $._var_declaration,
      $.assignment,
      $._return_statement,
      $.if_statement,
      $.while_loop,
      $.repeat_until_loop,
      $.for_loop,
      $.foreach_loop,
      $.switch_statement,
      seq($.method_call, $.statement_end),
    ),
    _var_declaration: $ => seq(
      choice($.var_declaration, $.var_declaration_with_assignment),
    ),
    var_declaration: $ => seq(
      $.declaration_keyword,
      $.identifier,
      $.var_type_declaration,
      $.statement_end
    ),
    var_declaration_with_assignment: $ => seq(
      $.declaration_keyword,
      $.identifier,
      alias($.assignable_var_type_declaration, $.var_type_declaration),
      $.var_declaration_assignment,
      $.statement_end
    ),
    declaration_keyword: _ => 'var',
    var_type_declaration: $ => choice(
      $.non_assignable_var_type_declaration,
      $.assignable_var_type_declaration
    ),
    assignable_var_type_declaration: $ => seq(
      ':',
      $.assignable_var_type,
    ),
    non_assignable_var_type_declaration: $ => seq(
      ':',
      $.non_assignable_var_type,
    ),
    var_declaration_assignment: $ => seq(
      $.assignment_operator,
      $.expression
    ),
    assignment: $ => seq(
      seq($.identifier, optional(seq('[', $.expression, ']'))),
      $.assignment_operator,
      $.expression,
      $.statement_end
    ),

    _return_statement: $ => choice($.return_statement, $.return_statement_with_expression),
    return_statement: $ => seq(
      $.return_keyword,
      $.statement_end,
    ),
    return_statement_with_expression: $ => seq(
      $.return_keyword,
      $.expression,
      $.statement_end,
    ),
    return_keyword: _ => 'return',

    if_statement: $ => seq(
      'if',
      $.expression,
      $.code_block,
      optional(seq(
        'else',
        $.code_block)),
    ),
    while_loop: $ => seq(
      'while',
      $.expression,
      $.code_block
    ),
    repeat_until_loop: $ => seq(
      'repeat',
      $.code_block,
      'until',
      $.expression,
      $.statement_end
    ),
    for_loop: $ => seq(
      'for',
      $.identifier,
      $.assignment_operator,
      $.expression,
      choice('to', 'downto'),
      $.expression,
      $.code_block
    ),
    foreach_loop: $ => seq(
      'foreach',
      $.identifier,
      'in',
      $.identifier,
      $.code_block
    ),

    switch_statement: $ => seq(
      $.switch_keyword,
      $.expression,
      $.switch_begin,
      $.switch_cases,
      $.switch_end
    ),
    switch_keyword: _ => 'switch',
    switch_cases: $ => seq(
      repeat1($.switch_case),
      optional($.switch_default),
    ),
    switch_begin: _ => '{',
    switch_end: _ => '}',
    switch_case: $ => seq(
      $.case_keyword,
      $.expression,
      repeat(seq(',', $.expression)),
      $.case_begin,
      $.case_body
    ),
    case_keyword: _ => 'case',
    case_begin: _ => ':',
    switch_default: $ => seq(
      $.case_default_keyword,
      $.case_default_begin,
      optional($.case_body)
    ),
    case_default_keyword: _ => 'default',
    case_default_begin: _ => ':',
    case_body: $ => choice($._code_statement, $.multiple_statements),
    multiple_statements: $ => seq(
      $._code_statement,
      repeat1($._code_statement),
    ),

    assignment_operator: _ => choice(...assignment_operators),

    expression: $ => prec(1, choice(
      $.unary_expression,
      $.binary_expression,
      $.parenthesized_expression,
      $.range_expression,
      $.method_call,
      $.identifier,
      $.number_literal,
      $.boolean_literal,
      $.string_literal,
      $.date_literal,
    )),

    unary_expression: $ => get_unary_expression($.expression, unary_operators),
    binary_expression: $ => get_binary_expression($.expression),
    property_unary_expression: $ => get_unary_expression($._property_value_recursive, property_unary_expression_operators),
    property_binary_expression: $ => get_binary_expression($._property_value_recursive),

    method_call: $ => seq(
      $.identifier,
      $.argument_list,
    ),
    argument_list: $ => seq(
      '(',
      optional($.expression),
      repeat(seq(',', $.expression)),
      ')'
    ),

    identifier: $ => choice(
      $._normal_identifier,
      $.identifier_in_quotes,
      prec.left(seq($.identifier, '.', $.identifier)),
      prec.left(seq($.identifier, '::', $.identifier)),
      prec.left(seq($.identifier, '[', $.expression, ']')),
      prec.left(seq('@', $.expression)),
    ),
    _normal_identifier: _ => token(/[a-zA-Z_]+[0-9_a-zA-Z]*/),
    identifier_in_quotes: _ => token(/"(("")|([^"]))*"/),
    statement_end: _ => ';',
    number_literal: _ => token(number_literal),
    integer_literal: _ => token(integer_literal),
    boolean_literal: _ => token(boolean_literal),
    date_literal: _ => token(/\d+(\.\d+)?(DT|D|T)/),
    string_literal: _ => token(/'(('')|([^']))*'/),
    parenthesized_expression: $ => get_parenthesized_expression($.expression),
    range_expression: $ => get_range_expression($.expression),
    property_parenthesized_expression: $ => get_parenthesized_expression($._property_value_recursive),
    property_range_expression: $ => get_range_expression($._property_value_recursive),

    // http://stackoverflow.com/questions/13014947/regex-to-match-a-c-style-multiline-comment/36328890#36328890
    comment: _ => token(choice(
      seq('//', /.*/),
      seq(
        '/*',
        /[^*]*\*+([^/*][^*]*\*+)*/,
        '/',
      ),
    )),
    preprocessor: _ => token(/#.*\n/),
  }
});

function get_unary_expression(inner_expression, operators) {
  return prec(PREC.unary, seq(
    choice(...operators),
    inner_expression,
  ));
}
function get_binary_expression(inner_expression) {
  const table = [
    [PREC.multiplicative, choice(...multiplicative_operators)],
    [PREC.additive, choice(...additive_operators)],
    [PREC.comparative, choice(...comparative_operators)],
    [PREC.and, and_operator],
    [PREC.or, choice(...or_operators)],
  ];
  return choice(...table.map(([precedence, operator]) =>
    prec.left(precedence, seq(
      inner_expression,
      operator,
      inner_expression,
    )),
  ));
}
function get_parenthesized_expression(inner_expression) {
  return seq('(', inner_expression, ')');
}
function get_range_expression(inner_expression) {
  return prec.left(seq(inner_expression, '..', inner_expression));
}
