package com.laba1.customer_service.enitity;

import com.fasterxml.jackson.annotation.JsonProperty;

public class customer {
    @JsonProperty("Customer number")
    private long clientId;
    @JsonProperty("Customer name")
    private String name;
    @JsonProperty("Customer surename")
    private String surename;
    @JsonProperty("Customer phone_number")
    private String phone_number;
    @JsonProperty("Customer city")
    private String city;
    @JsonProperty("Customer addres")
    private String addres;

    public customer(long clientId, String name, String surename, String phone_number, String city, String addres) {
        this.clientId = clientId;
        this.name = name;
        this.surename = surename;
        this.phone_number = phone_number;
        this.city = city;
        this.addres = addres;
    }

    public long getClientId() {
        return clientId;
    }

    public void setClientId(long clientId) {
        this.clientId = clientId;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getSurename() {
        return surename;
    }

    public void setSurename(String surename) {
        this.surename = surename;
    }

    public String getPhone_number() {
        return phone_number;
    }

    public void setPhone_number(String phone_number) {
        this.phone_number = phone_number;
    }

    public String getCity() {
        return city;
    }

    public void setCity(String city) {
        this.city = city;
    }

    public String getAddres() {
        return addres;
    }

    public void setAddres(String addres) {
        this.addres = addres;
    }


}
