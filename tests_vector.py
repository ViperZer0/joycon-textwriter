import unittest
from vector import Vector

class TestVector(unittest.TestCase):

    def test_blank_constructor(self):
        vec = Vector()
        self.assertDictEqual(vec.data,{}, "Dict should be empty")

    def test_non_blank_constructor(self):
        vec = Vector({1: 1})
        self.assertDictEqual(vec.data, {1: 1}, "Dict should match the dict passed to the constructor")

    def test_add_same_key(self):
        vec1 = Vector({1: 1})
        vec2 = Vector({1: 2})
        vec3 = vec1 + vec2
        self.assertDictEqual(vec3.data, {1: 3}, "vector should add two keys together successfully!")

    def test_add_different_keys(self):
        vec1 = Vector({1: 1})
        vec2 = Vector({'a': 4})
        vec3 = vec1 + vec2
        self.assertDictEqual(vec3.data, {1:1,'a': 4}, "Vector should concat two different keys!")

    def test_add_dict_to_vect(self):
        vec1 = Vector({1:1})
        vec2 = vec1 + {1: 1}
        self.assertDictEqual(vec2.data, {1: 2}, "Vector should add vector and dict correctly!")

    def test_add_strings(self):
        vec1 = Vector({"a": "this is a string"})
        vec2 = Vector({"a": "this is another string"})
        vec3 = vec1 + vec2
        self.assertDictEqual(vec3.data, {"a": "this is a stringthis is another string"}, "Vector should concat strings!")

    def test_add_empty_vectors(self):
        vec1 = Vector()
        vec2 = Vector()
        vec3 = vec1 + vec2
        self.assertDictEqual(vec3.data, {}, "Adding two empty vectors should result in empty vector!")

if __name__ == "__main__":
    unittest.main()
