import java.time.LocalDateTime;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.function.Supplier;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.stream.Collector;
import java.util.stream.Collectors;

public class JavaStream {
    public static void main(String[] args) {

        SayHello sayHello = s -> System.out.println(s);
        sayHello.sayHello("哈哈 哈哈哈哈");

        Person ps = Person.create(Person::new);
        ps.setName("name");
        ps.say("Hello");
       System.out.println(Optional.ofNullable(null).isPresent());
       System.out.println(Integer.MIN_VALUE + 1);
       int[] arr1 = {1,2,3};
       int[] arr2 = {4,5,6,7};

       System.arraycopy(arr2, 0, arr1, 1, 2);
 
       System.out.println(arr1);
       Pattern pattern = Pattern.compile("\\.(xls$|xlsx$)");
       Matcher matcher = pattern.matcher("123.xlsx");

       System.err.println(matcher.find());

    }

}
@FunctionalInterface
interface SayHello {
    default void hello(){

    }
    default void sy(){};
    static void say(){};
    void sayHello(String msg);
}

class Person{
    private String name;
    private Integer age;

    public static Person create(Supplier<Person> sp){
        return sp.get();
    }
    public void say(String msg){
         System.out.println(msg+","+getName());
    }
    public String getName() {
        return name;
    }
    public void setName(String name) {
        this.name = name;
    }
    public Integer getAge() {
        return age;
    }

    public void setAge(Integer age) {
        this.age = age;
    }

}