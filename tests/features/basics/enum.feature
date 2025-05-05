Feature: Enum Attributes
  Panduza client must be able to manage enum attributes

  Background:
    Given a reactor connected on a test platform

  Scenario: Manage RW enum attribute
    Given the enum attribute rw "enum/rw"
    When I set rw enum to "test 1"
    Then the rw enum value is "test 1"
    When I set rw enum to "test 2"
    Then the rw enum value is "test 2"

  Scenario: Manage WO & RO enum attributes
    Given the enum attribute wo "enum/wo"
    Given the enum attribute ro "enum/ro"
    When I set wo enum to "test 1"
    Then the ro enum value is "test 1"
    When I set wo enum to "test 2"
    Then the ro enum value is "test 2"
