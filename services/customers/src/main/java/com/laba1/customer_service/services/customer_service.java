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

    customer_repository reposit = new customer_repository() {
        @Override
        public List<customer> findAll() {
            return null;
        }

        @Override
        public List<customer> findAll(Sort sort) {
            return null;
        }

        @Override
        public List<customer> findAllById(Iterable<Long> longs) {
            return null;
        }

        @Override
        public <S extends customer> List<S> saveAll(Iterable<S> entities) {
            return null;
        }

        @Override
        public void flush() {

        }

        @Override
        public <S extends customer> S saveAndFlush(S entity) {
            return null;
        }

        @Override
        public <S extends customer> List<S> saveAllAndFlush(Iterable<S> entities) {
            return null;
        }

        @Override
        public void deleteAllInBatch(Iterable<customer> entities) {

        }

        @Override
        public void deleteAllByIdInBatch(Iterable<Long> longs) {

        }

        @Override
        public void deleteAllInBatch() {

        }

        @Override
        public customer getOne(Long aLong) {
            return null;
        }

        @Override
        public customer getById(Long aLong) {
            return null;
        }

        @Override
        public customer getReferenceById(Long aLong) {
            return null;
        }

        @Override
        public <S extends customer> List<S> findAll(Example<S> example) {
            return null;
        }

        @Override
        public <S extends customer> List<S> findAll(Example<S> example, Sort sort) {
            return null;
        }

        @Override
        public Page<customer> findAll(Pageable pageable) {
            return null;
        }

        @Override
        public <S extends customer> S save(S entity) {
            return null;
        }

        @Override
        public Optional<customer> findById(Long aLong) {
            return Optional.empty();
        }

        @Override
        public boolean existsById(Long aLong) {
            return false;
        }

        @Override
        public long count() {
            return 0;
        }

        @Override
        public void deleteById(Long aLong) {

        }

        @Override
        public void delete(customer entity) {

        }

        @Override
        public void deleteAllById(Iterable<? extends Long> longs) {

        }

        @Override
        public void deleteAll(Iterable<? extends customer> entities) {

        }

        @Override
        public void deleteAll() {

        }

        @Override
        public <S extends customer> Optional<S> findOne(Example<S> example) {
            return Optional.empty();
        }

        @Override
        public <S extends customer> Page<S> findAll(Example<S> example, Pageable pageable) {
            return null;
        }

        @Override
        public <S extends customer> long count(Example<S> example) {
            return 0;
        }

        @Override
        public <S extends customer> boolean exists(Example<S> example) {
            return false;
        }

        @Override
        public <S extends customer, R> R findBy(Example<S> example, Function<FluentQuery.FetchableFluentQuery<S>, R> queryFunction) {
            return null;
        }
    };

    public List<customer> getAllCustomers () {
        return reposit.findAll();
    }

    public customer getById (long id) {
        return reposit.findById(id).get();
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
