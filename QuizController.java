import org.springframework.web.bind.annotation.*;
import java.util.*;

@RestController
@RequestMapping("/quiz")
@CrossOrigin
public class QuizController {

    private List<Question> questions = List.of(
        new Question("Qual a tensão padrão no Brasil?", new String[]{"110V", "127V", "220V", "380V"}, 1),
        new Question("Unidade de potência?", new String[]{"Volt", "Ampere", "Watt", "Ohm"}, 2),
        new Question("Lei de Ohm?", new String[]{"V=IR", "P=VI", "I=VR", "R=VI"}, 0),
        new Question("Fator de potência ideal?", new String[]{"0", "0.5", "1", "2"}, 2),
        new Question("Corrente contínua é?", new String[]{"AC", "DC", "Hz", "VA"}, 1),
        new Question("Unidade de resistência?", new String[]{"Watt", "Volt", "Ohm", "Tesla"}, 2),
        new Question("Potência aparente?", new String[]{"W", "VA", "VAR", "Hz"}, 1),
        new Question("Frequência no Brasil?", new String[]{"50Hz", "60Hz", "120Hz", "220Hz"}, 1),
        new Question("Capacitor armazena?", new String[]{"Corrente", "Energia elétrica", "Calor", "Magnetismo"}, 1),
        new Question("Símbolo da corrente?", new String[]{"V", "I", "R", "P"}, 1)
        new Question("Quem descobriu o Brasil?", new String[]{"Pedro Alvares Cabral", "Pedro Américo", "José de Alencar", "Frei Caneca"}, 0)
    );

    @GetMapping
    public List<Question> getRandomQuestions() {
        Collections.shuffle(questions);
        return questions.subList(0, 10);
    }
}
