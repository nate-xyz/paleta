from gi.repository import GObject, Adw, GLib

class Color(GObject.GObject):
    __gtype_name__ = 'Color'

    def __init__(self, id, r, g, b, a, hex) -> None:
        super().__init__()
        self.id = id 
        self.red = r 
        self.green = g
        self.blue = b 
        self.alpha = a 
        self.hex = hex 
        self.rgb = (r,g,b)
        self.rgba = (r,g,b,a)
        self.rgb_name = "rgb{}".format(self.rgb)

class Palette(GObject.GObject):
    __gtype_name__ = 'Palette'

    def __init__(self, id, name) -> None:
        super().__init__()
        self.id = id 
        self.name = name 
        self.colors = set()

    def add_color(self, color: Color):
        self.colors.add(color)

    def remove_color(self, color: Color):
        self.colors.remove(color)


class Model(GObject.GObject):
    __gtype_name__ = 'Model'

    __gsignals__ = {
        'populated': (GObject.SignalFlags.RUN_FIRST, None, ()),
    }


    def __init__(self, db) -> None:
        super().__init__()
        self.db = db 

        self.PALETTES = dict()
        self.COLORS = dict()

    def reset_model(self):
        self.PALETTES = dict()
        self.COLORS = dict()

    def get_palettes(self) -> dict:
        return self.PALETTES

    def get_palette(self, id) -> Palette:
        return self.PALETTES[id]

    def get_colors(self) -> dict:
        return self.COLORS

    def get_color(self, id) -> Color:
        return self.COLORS[id]

######
# POPULATE APP MODEL from DATABASE
######

    def populate(self):
            self.reset_model()  #reset all maps

            self.populate_colors()
            self.populate_palettes()
            self.populate_color_palette_join()

            self.emit('populated')

    def populate_colors(self):
        colors = self.db.query_colors()
        if colors == []:
            return
        for id, r, g, b, a, hex in colors:
            try:
                color = Color(id, r, g, b, a, hex)
                self.COLORS[id] = color
            except Exception as e:
                print(e)


    def populate_palettes(self):
        palettes = self.db.query_palettes()
        if palettes == []:
            return
        for id, name in palettes:
            try:
                palette = Palette(id, name)
                self.PALETTES[id] = palette

            except Exception as e:
                print(e)
   
    
    def populate_color_palette_join(self):
        joins = self.db.query_palette_color_junction()
        if joins == []:
            return
        for id, palette_id, color_id in joins:
            
            try:
                palette = self.get_palette(palette_id)
                color = self.get_color(color_id)
                palette.add_color(color)

            except Exception as e:
                print(e)
    