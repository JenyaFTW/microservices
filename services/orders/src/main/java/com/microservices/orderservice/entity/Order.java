package com.microservices.orderservice.entity;

import javax.persistence.*;
import java.util.List;

@Entity
@Table(schema = "orders")
public class Order {

    @Id()
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column
    private Long clientId;
    @Column
    @ElementCollection
    @CollectionTable(schema = "orders")
    private List<Long> itemId;

    public Order() {

    }

    public Order(Long clientId, List<Long> itemId) {
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