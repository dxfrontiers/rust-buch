package de.digitalfrontiers.rustbuch.dash;

import java.io.Serializable;

import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class Data implements Serializable {

    public Data(String name, int age) {
        this.name = name;
        this.age = age;
    }

    String name;
    int age;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }
}
