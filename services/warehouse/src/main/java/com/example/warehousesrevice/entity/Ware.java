package com.example.warehousesrevice.entity;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import javax.persistence.Entity;
import javax.persistence.GeneratedValue;
import javax.persistence.GenerationType;
import javax.persistence.Id;

@JsonSerialize
@Entity
public class Ware {

    public Ware ( String name, double price) {
        this.name = name;
        this.price = price;
    }


    @JsonProperty
    @Id ()
    @GeneratedValue(strategy = GenerationType.AUTO)
    private long id;

    @JsonProperty
    private String name;

    @JsonProperty
    private double price;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public double getPrice() {
        return price;
    }

    public void setPrice(double price) {
        this.price = price;
    }

    public Ware () {

    }


}
