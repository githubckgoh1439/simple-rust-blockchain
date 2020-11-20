Feature: Encode messages and write them to a file.

  Scenario: Encode a simple alice- message.
    Given I have an Encoder initialized with input "alice!"
    Then I should see "alice!" in the Encoder's input field
    When I Encode the Encoder's input
    Then testfile.txt exists
    And testfile.txt is not empty
    When I decrypt testfile.txt
    Then the Decoded result should be "alice!"