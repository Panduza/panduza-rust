Feature: Enum Attributes
  Panduza client must be able to manage enum attributes

  Background:
    Given a reactor connected on a test platform

  Scenario: Manage RW enum attribute
    Given the enum attribute rw "enum/rw"
    When I set rw enum to "Antoine"
    Then the rw enum value is "Antoine"
    When I set rw enum to "Edmundo"
    Then the rw enum value is "Edmundo"

  Scenario: Manage WO & RO enum attributes
    Given the enum attribute wo "enum/wo"
    Given the enum attribute ro "enum/ro"
    When I set wo enum to "Antoine"
    Then the ro enum value is "Antoine"
    When I set wo enum to "Edmundo"
    Then the ro enum value is "Edmundo"
