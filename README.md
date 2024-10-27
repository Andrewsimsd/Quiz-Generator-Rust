# Quiz-Generator-Rust
Generates randomized quizzes from a question bank such that each quiz is unique while allowing for a reduced set of answer keys.
# Requirements
1. Question banks will be stored in an XML file, including links to images for reference.
2. There must be a GUI for modifying the question bank (addition, deletion, editing).
3. The number of tests to be generated is defined by the user.
4. Each test generated must be unique with respect to the order of questions, and the order of the answers for each question.
5. The user will have the option to use the same set of questions for each test, or allow for the questions to be drawn from the question bank without randomly (i.e. if the question bank has 100 questions, the user may specify that each test will contain questions 1-20, or a random sampling)
6. The order of the answers for each test must be limited to a number provided by the user (i.e. when using scantron cards only five keys are available. so only 5 unique answer sqeuences will be generated.)
7. Quizzes will be given a coded label such that the corresponding answer key may be determined easily given hidden information.
