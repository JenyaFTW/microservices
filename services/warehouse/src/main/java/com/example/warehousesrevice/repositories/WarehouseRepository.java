package com.example.warehousesrevice.repositories;

import com.example.warehousesrevice.entity.Ware;
import org.springframework.stereotype.Repository;

import java.util.ArrayList;
import java.util.List;

public class WarehouseRepository {

    List<Ware> wares = new ArrayList<> ();

    public WarehouseRepository () {
        wares.add (new Ware (0, "Phone", 200.0));
        wares.add (new Ware (1, "Computer", 500.0));
        wares.add (new Ware (2, "Laptop", 300.0));
    }

    public List<Ware> getAllWares () {
        return wares;
    }

}
