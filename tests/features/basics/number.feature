Feature: Number Attributes
  Panduza client must be able to manage number attributes

  Background:
    Given a reactor connected on a test platform

  Scenario: Manage RW number attribute
    Given the number attribute rw "number/rw"
    When I set rw number to 10.6
    Then the rw number value is 10.6
    When I set rw number to 20.9
    Then the rw number value is 20.9

  Scenario: Manage WO & RO number attributes
    Given the number attribute wo "number/wo"
    Given the number attribute ro "number/ro"
    When I set wo number to 9.5
    Then the ro number value is 9.5
    When I set wo number to 95269.58
    Then the ro number value is 95269.58
