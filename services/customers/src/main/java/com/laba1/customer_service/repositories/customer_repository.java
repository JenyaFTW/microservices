package com.laba1.customer_service.repositories;

import com.laba1.customer_service.enitity.customer;
import org.springframework.stereotype.Repository;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface customer_repository extends JpaRepository<customer, Long>, CrudRepository<customer, Long> {

}