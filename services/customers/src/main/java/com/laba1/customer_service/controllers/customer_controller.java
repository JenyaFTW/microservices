package com.laba1.customer_service.controllers;

import com.laba1.customer_service.enitity.customer;
import com.laba1.customer_service.services.customer_service;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RequestMapping("/api/customer")
@RestController
public class customer_controller {

    @Autowired
    private customer_service CustomService = new customer_service();

    @GetMapping()
    public ResponseEntity<List<customer>> getAllCustomers () {
        System.out.println (CustomService.getAllCustomers());
        return ResponseEntity.ok (CustomService.getAllCustomers());
    }

    @GetMapping("/{id}")
    @ResponseBody
    public customer getCustomerrById(@PathVariable long id) {
        return CustomService.getById(id);
    }

    @PostMapping()
    public void createOrder (@RequestBody customer customer) {
        CustomService.addCustomer(customer);
    }

    @DeleteMapping("/{id}")
    public void deleteOrder (@PathVariable Long id) {
        CustomService.deleteCustomer(id);
    }

    @PutMapping("/{id}")
    public void updateOrder (@RequestBody customer customer, @PathVariable Long id) {
        if (customer.getId() == null) customer.setId(id);
        CustomService.updateCustomer(customer);
    }
}


