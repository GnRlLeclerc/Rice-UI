/**
 * @file Rice grammar for tree-sitter
 * @author Thibaut de Saivre <thibaut2saivre@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "rice",

  extras: ($) => [$.comment, $.docstring, /\s+/],

  rules: {
    source_file: ($) => repeat($._block),

    _block: ($) => choice($.element, $.property),

    // Starts with double slash
    comment: ($) => seq("//", /[^\n]*/),

    // Starts with triple slash
    docstring: ($) => seq("///", /[^\n]*/),

    // Starts with capital letter
    classname: ($) => /[A-Z][a-zA-Z0-9_]*/,
    // Starts with lowercase letter
    propname: ($) => /[a-z][a-zA-Z0-9_]*/,

    // Either a classname and content, or omit the parentheses if no content
    element: ($) =>
      choice(seq($.classname, "{", repeat($._block), "}"), $.classname),

    property: ($) => choice(seq($.propname, ":", $._value), $.propname),

    // ************************************************* //
    //                        VALUES                     //
    // ************************************************* //

    // Value for a property
    _value: ($) =>
      choice($.boolean, $.string, $.pixels, $.fraction, $.percentage),

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
