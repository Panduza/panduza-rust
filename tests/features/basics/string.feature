Feature: String Attributes
  Panduza client must be able to manage string attributes

  Background:
    Given a reactor connected on a test platform

  Scenario: Manage RW string attribute
    Given the string attribute rw "string/rw"
    When I set rw string to "test 1"
    Then the rw string value is "test 1"
    When I set rw string to "test 2"
    Then the rw string value is "test 2"

  Scenario: Manage WO & RO string attributes
    Given the string attribute wo "string/wo"
    Given the string attribute ro "string/ro"
    When I set wo string to "test 1"
    Then the ro string value is "test 1"
    When I set wo string to "test 2"
    Then the ro string value is "test 2"
# Feature: Enum Attributes
#   Panduza client must be able to manage enum attributes
#   Background:
#     Given a reactor connected on a test platform
#   Scenario: Manage RW enum attribute
#     Given the enum attribute rw "enum/rw"
#     When I set rw enum to "Antoine"
#     Then the rw enum value is "Antoine"
#     When I set rw enum to "Edmundo"
#     Then the rw enum value is "Edmundo"
#   Scenario: Manage WO & RO enum attributes
#     Given the enum attribute wo "enum/wo"
#     Given the enum attribute ro "enum/ro"
#     When I set wo enum to "Antoine"
#     Then the ro enum value is "Antoine"
#     When I set wo enum to "Edmundo"
#     Then the ro enum value is "Edmundo"
