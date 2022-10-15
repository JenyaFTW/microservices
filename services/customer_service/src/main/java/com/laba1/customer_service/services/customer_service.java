package com.laba1.customer_service.services;

import com.laba1.customer_service.repositories.customer_repository;
import com.laba1.customer_service.enitity.customer;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class customer_service {

    @Autowired
    private customer_repository CustomRep;

    public List<customer> getCustomers () {
        return CustomRep.getAllCustomers ();
    }

    public customer getCustomerById(long id) {
        return CustomRep.getCustomerById(id);
    }

}