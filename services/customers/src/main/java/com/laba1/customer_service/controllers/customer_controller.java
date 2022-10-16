package com.laba1.customer_service.controllers;

import com.laba1.customer_service.enitity.customer;
import com.laba1.customer_service.services.customer_service;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;


@RestController
public class customer_controller {

    @Autowired
    private customer_service CustomService = new customer_service();

    @RequestMapping("/api/customers")
    public List<customer> getAllCustomers() {
        return CustomService.getCustomers();
    }

    @RequestMapping("/api/customer/{id}")
    public customer getCustomerById(@PathVariable long id) {
        return CustomService.getCustomerById(id);
    }
}


