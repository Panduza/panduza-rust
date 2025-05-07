Feature: Boolean Attributes
  Panduza client must be able to manage boolean attributes

  Background:
    Given a reactor connected on a test platform

  Scenario: Manage RW boolean attribute
    Given the boolean attribute rw "boolean/rw"
    When I set rw boolean to true
    Then the rw boolean value is true
    When I set rw boolean to false
    Then the rw boolean value is false

  Scenario: Manage WO & RO boolean attributes
    Given the boolean attribute wo "boolean/wo"
    Given the boolean attribute ro "boolean/ro"
    When I set wo boolean to true
    Then the ro boolean value is true
    When I set wo boolean to false
    Then the ro boolean value is false

  Scenario: Manage an instance error during a boolean attribute operation
    Given the boolean attribute wo "boolean/error"
    Then the status attribute must indicate running for all instances
    When I set wo boolean to true
    Then the status attribute must indicate an error for one instance
