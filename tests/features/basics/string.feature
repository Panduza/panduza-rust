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
