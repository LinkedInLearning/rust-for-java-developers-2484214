# Rust for Java Developers
This is the repository for the LinkedIn Learning course Rust for Java Developers. The full course is available from [LinkedIn Learning][lil-course-url].

![Rust for Java Developers][lil-thumbnail-url] 

Are you a Java programmer looking to move into Rust? While both are popular languages, Rust has a number of performance and security advantages—programs use less memory, are easier to develop, and there’s no garbage collector to fight with. In this course, instructor Tim McNamara shows you how to use your Java experience as a springboard for learning Rust. Tim gives you a tour of Rust’s type system, examines its safety features, and describes how error handling works without exceptions.

## Instructions

This repository has branches for each of the videos in the course. You can use the branch pop up menu in github to switch to a specific branch and take a look at the course at that stage, or you can add `/tree/BRANCH_NAME` to the URL to go to the branch you want to access.

## Branches

The branches are structured to correspond to the videos in the course. The naming convention is `CHAPTER#_MOVIE#`, followed by either `b` or `e` to indicate "beginning" and "end", respectively. For example, the branch named `02_03e` corresponds to the solution video for the challenge presented in the second chapter as the third video in that chapter.

| Chapter        | Branch |
| -------------- | ------ |
| Challenge: Data Processing | [`01_06b`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/01_06b) |
| Solution: Data Processing | [`01_06e`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/01_06e) |
| Challenge: Linked List | [`02_05b`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/02_05b) |
| Solution: Linked List | [`02_05e`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/02_05e) |
| Challenge: Analyze a messy CSV file | [`03_03b`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/03_03b) |
| Solution: Analyze a messy CSV file  | [`03_03e`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/03_03e) |
| Challenge: Count words with map/reduce | [`04_04b`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/04_04b) |
| Solution: Count words with map/reduce | [`04_04e`](https://github.com/LinkedInLearning/rust-for-java-developers-2484214/tree/04_04e) |

The `b` branch contains the code as it is at the beginning of the movie. The `e` branch contains the code as it is at the end of the movie. The `main` branch holds the final state of the code when in the course.


When switching from one exercise files branch to the next after making changes to the files, you may get a message like this:

    error: Your local changes to the following files would be overwritten by checkout:        [files]
    Please commit your changes or stash them before you switch branches.
    Aborting

To resolve this issue:
	
    Add changes to git using this command: git add .
	Commit changes using this command: git commit -m "some message"


## Installing

1. To use these exercise files, you must have the following installed:
	- Rust (version 1.70 or later) and cargo, available from https://rustup.org
3. Clone this repository into your local machine using the terminal (Mac), CMD (Windows), or a GUI tool like SourceTree.
4. Use a standard Rust workflow, e.g. `cargo build` and `cargo run` to compile and run programs.

### Instructor

Tim McNamara 
                            
Author, Software Architect

                            

Check out my other courses on [LinkedIn Learning](https://www.linkedin.com/learning/instructors/tim-mcnamara).

[lil-course-url]: https://www.linkedin.com/learning/rust-for-java-developers?dApp=59033956&leis=LAA
[lil-thumbnail-url]: https://media.licdn.com/dms/image/D560DAQHpAgZOm7eHew/learning-public-crop_288_512/0/1699387980723?e=2147483647&v=beta&t=5EjgnKAt3DPDXR-bpgmaPCXs_6rRXhZWkbF4QsX32cE

