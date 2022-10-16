package com.example.warehousesrevice.repositories;



import com.example.warehousesrevice.entity.Ware;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface WarehouseRepository extends JpaRepository<Ware, Long>, CrudRepository<Ware, Long> {
}
