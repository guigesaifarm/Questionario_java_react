import React, { useEffect, useState } from "react";

export default function QuizApp() {
  const [questions, setQuestions] = useState([]);
  const [answers, setAnswers] = useState({});
  const [result, setResult] = useState(null);

  useEffect(() => {
    fetch("http://localhost:8080/quiz")
      .then(res => res.json())
      .then(data => setQuestions(data));
  }, []);

  const handleAnswer = (qIndex, optionIndex) => {
    setAnswers({ ...answers, [qIndex]: optionIndex });
  };

  const calculate = () => {
    let score = 0;
    const details = questions.map((q, i) => {
      const correct = q.correctIndex;
      const user = answers[i];
      const isCorrect = user === correct;
      if (isCorrect) score += 2;

      return {
        question: q.question,
        correct: q.options[correct],
        user: q.options[user],
        isCorrect
      };
    });

    setResult({ score, details });
  };

  if (result) {
    return (
      <div>
        <h2>Nota final: {result.score}</h2>
        <table border="1">
          <thead>
            <tr>
              <th>Pergunta</th>
              <th>Sua resposta</th>
              <th>Correta</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            {result.details.map((d, i) => (
              <tr key={i}>
                <td>{d.question}</td>
                <td>{d.user}</td>
                <td>{d.correct}</td>
                <td>{d.isCorrect ? "✔" : "✘"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }

  return (
    <div>
      <h1>Quiz Eletrotécnica</h1>
      {questions.map((q, i) => (
        <div key={i}>
          <p>{i + 1}. {q.question}</p>
          {q.options.map((opt, j) => (
            <button key={j} onClick={() => handleAnswer(i, j)}>
              {opt}
            </button>
          ))}
        </div>
      ))}
      <button onClick={calculate}>Finalizar</button>
    </div>
  );
}
