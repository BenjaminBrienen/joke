const QUESTIONS_MAX_ALLOWED: usize = 4;

fn main()
{
	let questions = include_str!("../questions.txt").split('\n').collect();
	let answers = answer_questions(questions);
	println!("{answers:?}");
}

fn answer_questions(questions: Vec<&str>) -> Result<Vec<String>, ()>
{
	if questions.len() > QUESTIONS_MAX_ALLOWED
	{
		Err(kill_person_asking_too_many_questions())
	}
	else
	{
		let answers = questions
			.iter()
			.take(QUESTIONS_MAX_ALLOWED)
			.map(answer_question)
			.collect();
		Ok(answers)
	}
}

fn answer_question(question: &&str) -> String { todo!() }

fn kill_person_asking_too_many_questions() { todo!() }
