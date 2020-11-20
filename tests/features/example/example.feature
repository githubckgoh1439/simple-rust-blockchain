Feature: Basic functionality
  Scenario: An example scenario
   # Comment
    Given I am trying out Cucumber
    When I consider what I am doing
    Then I am interested in ATDD
    And we can implement rules with regex
    
  Scenario Outline: scenario with examples
   # Comment
    Given a number <num>
    Then twice that number should be <double>

    Examples:
      | num | double |
      |   2 |      4 |
      |   3 |      6 |

  Scenario: bar
   # Comment
    Given a thing
    When something goes wrong    

  Scenario: bar02
   # Comment
    Given a thing-bar02
    When something-bar02 goes wrong    

  Scenario: alice
   # Comment
    Given a thing
    When is a thing
