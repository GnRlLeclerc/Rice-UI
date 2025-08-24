/**
 * @file Rice grammar for tree-sitter
 * @author Thibaut de Saivre <thibaut2saivre@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "rice",

  extras: (_) => [/\s+/],

  conflicts: (_) => [],

  rules: {
    // For now, multiple declarations and components allowed
    source_file: ($) => repeat(choice($._decl, $.component, $.comment)),

    // ************************************************* //
    //                    DECLARATIONS                   //
    // ************************************************* //

    // Any declaration
    _decl: ($) => choice($.enum_decl, $.component_decl),

    // Enums
    enum_decl: ($) =>
      seq(
        optional($.docstring),
        "enum",
        $.classname,
        "{",
        repeat(choice($.enum_variant_decl, $.comment)),
        "}",
      ),

    // Components
    component_decl: ($) =>
      seq(
        optional($.docstring),
        "component",
        $.classname,
        "{",
        repeat($._block_decl),
        "}",
      ),

    // Block inside a component declaration
    _block_decl: ($) => choice($.property_decl, $.component, $.comment),

    // Property declaration, with Golang-style typing
    property_decl: ($) =>
      seq(
        optional($.docstring),
        $.propname,
        $.classname,
        optional($._default_value),
      ),

    enum_variant_decl: ($) => seq(optional($.docstring), $.identifier),

    _default_value: ($) => seq("=", $.value),

    // ************************************************* //
    //                      COMPONENT                    //
    // ************************************************* //

    // Using a component
    component: ($) => seq($.classname, "{", repeat($._block), "}"),

    // Inside a component, assign to properties or create child components
    _block: ($) => choice($.property, $.component, $.comment),

    // ************************************************* //
    //               KEYWORDS & IDENTIFIERS              //
    // ************************************************* //

    // Starts with triple slash
    docstring: (_) => prec.right(1, repeat1(seq("///", /[^\n]*/))),

    // Starts with double slash
    comment: (_) => prec.right(repeat1(seq("//", /[^\n]*/))),

    // Starts with capital letter
    classname: (_) => /[A-Z][a-zA-Z0-9_]*/,
    // Starts with lowercase letter
    propname: (_) => /[a-z][a-zA-Z0-9_]*/,
    // Starts with lowercase letter
    identifier: (_) => /[a-z][a-zA-Z0-9_]*/,

    property: ($) => choice(seq($.propname, ":", $.value), $.propname),

    // ************************************************* //
    //                        VALUES                     //
    // ************************************************* //

    // Value for a property. Identifiers allowed for now, but no expressions
    value: ($) =>
      choice(
        $.boolean,
        $.string,
        $.pixels,
        $.fraction,
        $.percentage,
        $.identifier,
      ),

    boolean: (_) => choice("true", "false"),

    // Anything between double quotes. Escaped quotes are allowed.
    string: (_) => /"(?:[^"\\]|\\.)*"/,

    // Pixel amounts (e.g. "10px", "20px")
    pixels: (_) => /[0-9]+px/,

    // Fraction amounts (e.g. 1fr, 2.5fr)
    fraction: (_) => /[0-9]+(?:\.[0-9]+)?fr/,

    // Percentage amounts (e.g. 50%, 50.5%)
    percentage: (_) => /[0-9]+(?:\.[0-9]+)?%/,
  },
});
