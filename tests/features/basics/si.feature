Feature: Si Attributes
  Panduza client must be able to manage si attributes

  Background:
    Given a reactor connected on a test platform

  Scenario: Manage RW si attribute
    Given the si attribute rw "si/rw"
    When I set rw si to 10.6
    Then the rw si value is 10.6
    When I set rw si to 20.9
    Then the rw si value is 20.9

  Scenario: Manage WO & RO si attributes
    Given the si attribute wo "si/wo"
    Given the si attribute ro "si/ro"
    When I set wo si to 9.5
    Then the ro si value is 9.5
    When I set wo si to 95269.58
    Then the ro si value is 95269.58
