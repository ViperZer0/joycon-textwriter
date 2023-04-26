class Vector:
    def __init__(self, data=None):
        if data == None:
            self.data = {}
        else:
            self.data = dict(data)


    # Add two vectors or vector-like objects.
    def __add__(self, other):
        # Extract contained data from Vector, otherwise treat it like a dict or something.
        other = other.data if isinstance(other, Vector) else other
        new_data = {}
        # Get all the keys from both vectors, iterate through them.
        for key in set(self.data).union(set(other)):
            if key in self.data and key in other:
                new_data[key] = self.data[key] + other[key]
            else:
                if key in self.data:
                    new_data[key] = self.data[key]
                elif key in other:
                    new_data[key] = other[key]
                else:
                    assert False, "This should never happen!"
        return Vector(new_data)

    # Division by scalar (not truncate)
    def __truediv__(self, other):
        pass

    def __mul__(self, other):
        pass
