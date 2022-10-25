package com.example.warehousesrevice.services;

import com.example.warehousesrevice.entity.Ware;
import com.example.warehousesrevice.repositories.WarehouseRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

@Service
public class WareService {


    @Autowired
    private WarehouseRepository warehouseRepository;


    public List<Ware> getAllWares () {
        return warehouseRepository.findAll();
    }

    public void addWare (Ware ware) {
        warehouseRepository.saveAndFlush(ware);
    }

    public void updateWare (Ware ware) {
        warehouseRepository.save(ware);
    }

    public void deleteWare (Ware ware) {
        warehouseRepository.delete(ware);
    }

    public Ware getWareById (Long id) {
        return warehouseRepository.findById(id).get();
    }



}
