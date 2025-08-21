/**
 * @file Rice grammar for tree-sitter
 * @author Thibaut de Saivre <thibaut2saivre@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

export default grammar({
  name: "rice",

  extras: ($) => [$.comment, $.docstring, /\s+/],

  conflicts: ($) => [[$.property, $.property_decl]],

  rules: {
    // For now, multiple declarations and components allowed
    source_file: ($) => repeat(choice($._decl, $.component)),

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
        repeat($.identifier),
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
    _block_decl: ($) => choice($.property_decl, $.property, $.component),

    // Property declaration, with Golang-style typing
    property_decl: ($) =>
      seq(
        optional($.docstring),
        $.propname,
        $.classname,
        optional($._default_value),
      ),

    _default_value: ($) => seq("=", $._value),

    // ************************************************* //
    //                      COMPONENT                    //
    // ************************************************* //

    // Using a component
    component: ($) => prec.left(seq($.classname, "{", repeat($._block), "}")),

    // Inside a component, assign to properties or create child components
    _block: ($) => choice($.property, $.component),

    // ************************************************* //
    //                    TODO                   //
    // ************************************************* //

    // Starts with double slash
    comment: ($) => seq("//", /[^\n]*/),

    // Starts with triple slash
    docstring: ($) => seq("///", /[^\n]*/),

    // Starts with capital letter
    classname: ($) => /[A-Z][a-zA-Z0-9_]*/,
    // Starts with lowercase letter
    propname: ($) => /[a-z][a-zA-Z0-9_]*/,
    // Starts with lowercase letter
    identifier: ($) => /[a-z][a-zA-Z0-9_]*/,

    property: ($) => choice(seq($.propname, ":", $._value), $.propname),

    // ************************************************* //
    //                        VALUES                     //
    // ************************************************* //

    // Value for a property. Identifiers allowed for now, but no expressions
    _value: ($) =>
      choice(
        $.boolean,
        $.string,
        $.pixels,
        $.fraction,
        $.percentage,
        $.identifier,
      ),

    boolean: ($) => choice("true", "false"),

    // Anything between double quotes. Escaped quotes are allowed.
    string: ($) => /"(?:[^"\\]|\\.)*"/,

    // Pixel amounts (e.g. "10px", "20px")
    pixels: ($) => /[0-9]+px/,

    // Fraction amounts (e.g. 1fr, 2.5fr)
    fraction: ($) => /[0-9]+(?:\.[0-9]+)?fr/,

    // Percentage amounts (e.g. 50%, 50.5%)
    percentage: ($) => /[0-9]+(?:\.[0-9]+)?%/,
  },
});
