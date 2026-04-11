from group_enhancer import PyIndividual
import numpy as np
import random

def roulette_wheel(population : list, fit_values : list): #función para seleccionar un individuo de la población usando la selección por ruleta
    
    total_fitness = sum(fit_values) #Sumar los valores fitness para calcular las probabilidades de selección
    probabilities = [fit / total_fitness for fit in fit_values] #Calcular las probabilidades de selección para cada individuo
    print("Selection probabilities: ", probabilities)
    
    cumulative_probabilities = np.cumsum(probabilities) #Calcular las probabilidades acumuladas para la selección por ruleta
    print("Cumulative probabilities: ", cumulative_probabilities)
    
    r = random.uniform(0, 1) #Generar un número aleatorio uniforme entre 0 y 1 para seleccionar un individuo
    for i, cumulative_probability in enumerate(cumulative_probabilities):
        if r < cumulative_probability: #Seleccionar el primer individuo cuya probabilidad acumulada sea mayor que el número aleatorio generado
            return population[i], i #Seleccionar el individuo correspondiente a la probabilidad acumulada
        


    