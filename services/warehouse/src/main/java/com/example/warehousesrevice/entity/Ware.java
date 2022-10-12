package com.example.warehousesrevice.entity;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

@JsonSerialize
public class Ware {

    public Ware (long id, String name, double price) {
        this.id = id;
        this.name = name;
        this.price = price;
    }


    @JsonProperty
    private long id;

    @JsonProperty
    private String name;

    @JsonProperty
    private double price;


    public Ware () {

    }


}
