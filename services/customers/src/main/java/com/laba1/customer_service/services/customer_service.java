package com.laba1.customer_service.services;

import com.laba1.customer_service.repositories.customer_repository;
import com.laba1.customer_service.enitity.customer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.domain.Example;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.domain.Sort;
import org.springframework.data.repository.query.FluentQuery;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;
import java.util.function.Function;

@Service
public class customer_service {

    private final customer_repository reposit;

    @Autowired
    public customer_service(customer_repository reposit) {
        this.reposit = reposit;
    }
    public List<customer> getAllCustomers () {
        return reposit.findAll();
    }

    public customer getById (long id) {
        return reposit.findById(id).orElse(null);
    }

    public void addCustomer (customer customer) {
        reposit.saveAndFlush(customer);
    }

    public void deleteCustomer (long id) {
        reposit.deleteById(id);
    }

    public void updateCustomer (customer customer) {
        reposit.save(customer);
    }
}