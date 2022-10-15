package com.laba1.customer_service.repositories;

import com.laba1.customer_service.enitity.customer;
import org.springframework.stereotype.Repository;

import java.util.ArrayList;
import java.util.List;

@Repository
public class customer_repository {
    List<customer> customers = new ArrayList<>();

    public customer_repository() {
        customers.add(new customer(0, "Bалерій", "Пономаренко", "+380501060273", "Хмельницьк", "вул. Космонавта Попова, 05"));
        customers.add(new customer(1, "Раїса", "Гнатюк", "+380930556861", "Київ", "пров. 40 років Перемоги, 13"));
        customers.add(new customer(2, "Олександра", "Кравченко", "+380502277997", "Ужгород", "вул. Лесі Українки, 20"));
        customers.add(new customer(3, "Віталій", "Кравчук", "+380937150679", "Житомир", "пров. Волкова, 74"));
        customers.add(new customer(4, "Іван", "Павлюк", "+380672840289", "Дніпропетровськ", "пл. Тараса Шевченка, 62"));
    }

    public List<customer> getAllCustomers() {
        return customers;
    }

    public customer getCustomerById(long id) {
        for (customer s : customers) {
            if (s.getClientId() == id) {
                return s;
            }
        }
        return null;
    }
}