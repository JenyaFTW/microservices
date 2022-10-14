package com.microservices.orderservice.entity;

import java.util.List;

public class Order {

    private Long id;

    private Long clientId;

    private List<Long> itemId;

    public Order() {

    }

    public Order(Long id, Long clientId, List<Long> itemId) {
        this.id = id;
        this.clientId = clientId;
        this.itemId = itemId;
    }

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    public Long getClientId() {
        return clientId;
    }

    public void setClientId(Long clientId) {
        this.clientId = clientId;
    }

    public List<Long> getItemId() {
        return itemId;
    }

    public void setItemId(List<Long> itemId) {
        this.itemId = itemId;
    }
}