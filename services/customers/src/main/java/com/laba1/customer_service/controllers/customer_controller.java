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

    private final customer_service CustomService;

    @Autowired
    public customer_controller(customer_service CustomService) {
        this.CustomService = CustomService;
    }

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
    public void addCustomer (@RequestBody customer customer) {
        CustomService.addCustomer(customer);
    }

    @DeleteMapping("/{id}")
    public void deleteCustomer (@PathVariable Long id) {
        CustomService.deleteCustomer(id);
    }

    @PutMapping("/{id}")
    public void updateCustomer (@RequestBody customer customer, @PathVariable Long id) {
        if (customer.getId() == null) customer.setId(id);
        CustomService.updateCustomer(customer);
    }
}


