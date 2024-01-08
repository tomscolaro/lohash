import os
import numpy as np
import csv
import sys
import argparse
from os import environ
from collections import defaultdict
from pprint import pprint
from datetime import date

if not os.environ.get('PYTHONHASHSEED'):
    os.environ['PYTHONHASHSEED'] = '1234'
    os.execv(sys.executable, ['python3'] + sys.argv)
hashseed = os.getenv('PYTHONHASHSEED')

class StringLoHash(object):
    def __init__(self, data_path, output_path, verbose=False, threshold=.2, k=3, num_hashes=10, num_buckets=10, saveHash=True, seed=42, writeType="default", global_group_id_start=0):
        if saveHash:
            print("With the Save Hash Option, you need to create a seed.. Your seed is currently {}...".format(seed))
            np.random.seed(seed)

  
        self.write_type = writeType

        self.k = k
        self.threshold = threshold
        self.num_hashes = num_hashes
        self.num_buckets = num_buckets
        self.hash_functions = [self._create_hash_function() for _ in range(num_hashes)]
        self.hash_table = {i: np.inf * np.ones(num_hashes) for i in range(num_buckets)}

        
        self.input = data_path
        self.output_path = output_path
        self.rows  = defaultdict(list)
        self.rows_used  = defaultdict(bool)
        self.vocab = {}
        self.verbose  = verbose
        self.global_group_id = global_group_id_start
    
    
    def _create_hash_function(self):
        a = np.random.randint(1, 500)
        b = np.random.randint(1, 500)
        return lambda x: hash(str(a * hash(x) + b)) % self.num_buckets

    def _minhash(self, data):
        minhash_values = np.inf * np.ones(self.num_hashes)
        for i, hash_function in enumerate(self.hash_functions):
            for value in data:
                hash_value = hash_function(value)
                minhash_values[i] = min(minhash_values[i], hash_value)
                # print(hash_value, minhash_values)
        return minhash_values

    def index(self, item_id, data):
        self.hash_table[item_id] = self._minhash(data)

    def query(self, query_data, threshold):
        minhash_values_query = self._minhash(query_data)

        results = []
        for item_id, minhash_values_candidate in self.hash_table.items():
            similarity = np.sum(minhash_values_query == minhash_values_candidate) / self.num_hashes
            if similarity >= threshold:
                results.append((item_id, similarity, minhash_values_candidate))
        return results

    
    def _add_vocab(self, str, k, type="default"):
        match type:
            case _:
                data = self.shingle(str, k)
        return data


    def shingle(self, str, k):
        """
        Generate shingles of length k from the input string.

        Parameters:
        - input_string (str): The input string.
        - k (int): The length of each shingle.

        Returns:
        - set: A set of shingled strings.
        """
        shingles_set = set()

        # Check if k is a valid value
        if k <= 0 or k > len(str):
            raise ValueError("Invalid value for k")

        # Generate shingles
        for i in range(len(str) - k + 1):
            shingle = str[i:i+k]
            shingles_set.add(shingle)

        return shingles_set
    

    def readData(self, path, id_col=0, data_col=2):
        with open(path, mode ='r', encoding='utf-8') as file:
            csvFile = csv.reader(file)
            i = 0 
            for lines in csvFile:
                if i >0:
                    self.rows[(lines[id_col], lines[id_col+1])].append([lines[data_col], lines[data_col +1]])
                    self.rows_used[(lines[id_col], lines[id_col+1])] = False
                    data = self._add_vocab(lines[data_col], self.k)
                    self.index((lines[id_col], lines[id_col+1]), data)
                i += 1
        return

    def generateGroups(self):
    
        match self.write_type:
            
            case "csv":
                self.genFile()
                print("Generating direct to CSV...")
                for i in self.rows:
                    if self.rows_used[i]:
                        continue
        
                    search = self.shingle(self.rows[i][0][0], self.k)
                    res = self.query(search, self.threshold)
                

                    if self.verbose:
                        print("test", self.rows[i],  "\nout", len(res), "\n\n")
                    self.diskWrite(self.rows[i], i, results=res)

            case _: 
                print("Generating in Default Mode...")
                for i in self.rows:
                    search = self.shingle(self.rows[i][0][0], self.k)
                    self.rows[i].append(self.query(search, self.threshold))
        return
    
    def writeGroups(self):
        today = date.today()
        with open(self.output_path, 'w', newline='',  encoding='utf-8') as file:
            writer = csv.writer(file)
            writer.writerow(["Data", "Data Name", "ID", "Measurement", "Group Name", "Group Generated ID", "Date Generated"]) #headers
            idx = 0 
            for i in self.rows:
                groupName = "Group {}".format(idx)
                for j in self.rows[i][1]:
                    if self.verbose:
                        print(self.rows,"\n")
                    writer.writerow([self.rows[j[0]][0][0], self.rows[j[0]][0][1], int(j[0][1]), j[1],  groupName, idx, today])
                idx+=1
        return
    
    def diskWrite(self, input_rows, id,  results):
        today = date.today()

        groupId = self.global_group_id    
        groupName = 'Group {}'.format(groupId)
        self.global_group_id +=1

        with open(self.output_path, 'a', newline='',  encoding='utf-8') as file:
            writer = csv.writer(file)
            for i in results:
                writer.writerow([self.rows[i[0]][0][0], self.rows[i[0]][0][1], int(i[0][1]), i[1], groupName, groupId, today]) #headers
                self.rows_used[i[0]] = True

        file.close()
      
        return
    

    def genFile(self):
        with open(self.output_path, 'w', newline='',  encoding='utf-8') as file:
            writer = csv.writer(file)
            writer.writerow(["Data", "Data Name", "ID", "Measurement", "Group Name", "Group Generated ID", "Date Generated"]) #headers
        file.close()

    def display(self):
        print("Displaying Results... {} rows are available".format(len(self.rows)))
        printlist = list(self.rows)[:10]
        
        for i in printlist:
            print(i, self.rows[i])
        return 
    
    def start(self):
        print("Reading data...")
        self.readData(self.input)
        print("Reading complete....")
        print("Generating Groups....")
        self.generateGroups()
        print("Groups Generated....")
