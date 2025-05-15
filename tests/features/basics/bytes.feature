Feature: Bytes Attributes
  Panduza client must be able to manage bytes attributes

  Background:
    Given a reactor connected on a test platform

  Scenario: Manage RW bytes attribute
    Given the bytes attribute rw "bytes/rw"
    When I set rw bytes to "test 1"
    Then the rw bytes value is "test 1"
    When I set rw bytes to "test 2"
    Then the rw bytes value is "test 2"

  Scenario: Manage WO & RO bytes attributes
    Given the bytes attribute wo "bytes/wo"
    Given the bytes attribute ro "bytes/ro"
    When I set wo bytes to "test 1"
    Then the ro bytes value is "test 1"
    When I set wo bytes to "test 2"
    Then the ro bytes value is "test 2"
