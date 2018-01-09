import * as Joi from 'joi';

export interface Calendar {
  year?: number;
  month?: number;
  week?: number;
  date_range?: [DateRangePart, DateRangePart];
}

const schemas = {
  year: Joi.number().integer().min(1989).max(2010),
  month: Joi.number().integer().min(1).max(12),
  week: Joi.number().integer().min(1).max(53),
};

export interface DateRangePart {
  month?: number;
  part?: number;
}

export namespace DateRangePart {
  export const schema = Joi.object().keys({
    month: schemas.month,
    part: Joi.number().integer().min(1).max(3),
  })
}

const manufacturers = ['bsi', 'fujitsu', 'hynix', 'kds', 'kss', 'lsi-logic', 'microchip', 'mitsumi', 'mosel-vitelic', 'nec', 'rohm', 'sharp', 'tdk', 'toshiba', 'xlink'];

export interface Chip extends Calendar {
  type?: string;
  label: string | null;
  manufacturer?: string;
  outlier?: boolean;
}

export namespace Chip {
  export const schema = Joi.object().keys({
    type: Joi.string(),
    label: Joi.string().allow(null).required(),
    manufacturer: Joi.string().only(manufacturers),
    year: schemas.year,
    month: schemas.month,
    week: schemas.week,
    outlier: Joi.boolean()
  });
}

export interface Metadata {
  serial: string;
  type: string;
}

export interface DmgMetadata extends Metadata {
  type: "DMG";
  color?: string;
  screws?: string;
  year?: number;
  month?: number;
  mainboard: {
    type: string;
    circled_letters?: string | null;
    extra_label?: string;
    stamp?: string;
    cpu?: Chip;
    work_ram?: Chip;
    video_ram?: Chip;
    amplifier?: Chip;
    crystal?: Chip;
  };
  lcd_board?: {
    type: string;
    circled_letters?: string;
    stamp?: string;
    year?: number;
    month?: number;
    column_driver?: Chip;
    row_driver?: Chip;
    regulator?: Chip;
  };
  power_board?: {
    type: string;
    label: string;
    year?: number;
    month?: number;
  };
  jack_board?: {
    type: string;
    extra_label?: string | null
  };
}

export namespace DmgMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('DMG'),
    color: Joi.string(),
    screws: Joi.string(),
    year: schemas.year,
    month: schemas.month,
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string().allow(null),
      extra_label: Joi.string(),
      stamp: Joi.string(),
      cpu: Chip.schema,
      work_ram: Chip.schema,
      video_ram: Chip.schema,
      amplifier: Chip.schema,
      crystal: Chip.schema,
    }),
    lcd_board: Joi.object().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string(),
      stamp: Joi.string(),
      year: schemas.year,
      month: schemas.month,
      column_driver: Chip.schema,
      row_driver: Chip.schema,
      regulator: Chip.schema,
    }),
    power_board: Joi.object().keys({
      type: Joi.string().required(),
      label: Joi.string().required(),
      year: schemas.year,
      month: schemas.month
    }),
    jack_board: Joi.object().keys({
      type: Joi.string().required(),
      extra_label: Joi.string().allow(null)
    })
  });
}

export interface SgbMetadata extends Metadata {
  type: "SGB";
  stamp?: string;
  mainboard: {
    type: string;
    circled_letters?: string;
    letter_at_top_right?: string;
    cpu?: Chip;
    icd2?: Chip;
    work_ram?: Chip;
    video_ram?: Chip;
    rom?: Chip;
    cic?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace SgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('SGB'),
    stamp: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string(),
      letter_at_top_right: Joi.string(),
      cpu: Chip.schema,
      icd2: Chip.schema,
      work_ram: Chip.schema,
      video_ram: Chip.schema,
      rom: Chip.schema,
      cic: Chip.schema,
      year: schemas.year,
      month: schemas.month
    })
  });
}

export interface MgbMetadata extends Metadata {
  type: "MGB";
  color?: string;
  year?: number;
  month?: number;
  mainboard: {
    type: string;
    circled_letters?: string | null;
    number_pair?: string;
    stamp?: string;
    year?: number;
    month?: number;
    date_range?: [DateRangePart, DateRangePart],
    cpu?: Chip;
    work_ram?: Chip;
    amplifier?: Chip;
    regulator?: Chip;
    crystal?: Chip;
  };
  lcd?: {
    column_driver?: Chip;
    row_driver?: Chip;
  };
}

export namespace MgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('MGB'),
    color: Joi.string(),
    year: schemas.year,
    month: schemas.month,
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string().allow(null),
      number_pair: Joi.string(),
      stamp: Joi.string(),
      year: schemas.year,
      month: schemas.month,
      date_range: Joi.array().ordered(DateRangePart.schema, DateRangePart.schema),
      cpu: Chip.schema,
      work_ram: Chip.schema,
      amplifier: Chip.schema,
      regulator: Chip.schema,
      crystal: Chip.schema,
    }),
    lcd: Joi.object().keys({
      column_driver: Chip.schema,
      row_driver: Chip.schema,
    }),
  });
}

export interface MglMetadata extends Metadata {
  type: "MGL";
  color?: string;
  year?: number;
  week?: number;
  mainboard: {
    type: string;
    circled_letters?: string | null;
    number_pair?: string;
    stamp?: string;
    year?: number;
    month?: number;
    date_range?: [DateRangePart, DateRangePart],
    cpu?: Chip;
    work_ram?: Chip;
    amplifier?: Chip;
    regulator?: Chip;
    crystal?: Chip;
    t1?: Chip;
  };
  lcd?: {
    column_driver?: Chip;
    row_driver?: Chip;
  };
}

export namespace MglMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('MGL'),
    color: Joi.string(),
    year: schemas.year,
    week: schemas.week,
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string().allow(null),
      number_pair: Joi.string(),
      stamp: Joi.string(),
      year: schemas.year,
      month: schemas.month,
      date_range: Joi.array().ordered(DateRangePart.schema, DateRangePart.schema),
      cpu: Chip.schema,
      work_ram: Chip.schema,
      amplifier: Chip.schema,
      regulator: Chip.schema,
      crystal: Chip.schema,
      t1: Chip.schema,
    }),
    lcd: Joi.object().keys({
      column_driver: Chip.schema,
      row_driver: Chip.schema,
    }),
  });
}

export interface Sgb2Metadata extends Metadata {
  type: "SGB2";
  stamp?: string;
  mainboard: {
    type: string;
    circled_letters?: string;
    letter_at_top_right?: string;
    crystal?: Chip;
    cpu?: Chip;
    icd2?: Chip;
    work_ram?: Chip;
    rom?: Chip;
    cic?: Chip;
    coil?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace Sgb2Metadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('SGB2'),
    stamp: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string(),
      letter_at_top_right: Joi.string(),
      crystal: Chip.schema,
      cpu: Chip.schema,
      icd2: Chip.schema,
      work_ram: Chip.schema,
      rom: Chip.schema,
      cic: Chip.schema,
      coil: Chip.schema,
      year: schemas.year,
      month: schemas.month
    })
  });
}

export interface CgbMetadata extends Metadata {
  type: "CGB";
  color?: string;
  year?: number;
  week?: number;
  mainboard: {
    type: string;
    circled_letters?: string | null;
    number_pair?: string;
    stamp?: string;
    year?: number;
    month?: number;
    date_range?: [DateRangePart, DateRangePart],
    cpu?: Chip;
    work_ram?: Chip;
    amplifier?: Chip;
    regulator?: Chip;
    crystal?: Chip;
  };
}

export namespace CgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('CGB'),
    color: Joi.string(),
    year: schemas.year,
    week: schemas.week,
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string().allow(null),
      number_pair: Joi.string(),
      stamp: Joi.string(),
      year: schemas.year,
      month: schemas.month,
      date_range: Joi.array().ordered(DateRangePart.schema, DateRangePart.schema),
      cpu: Chip.schema,
      work_ram: Chip.schema,
      amplifier: Chip.schema,
      regulator: Chip.schema,
      crystal: Chip.schema,
    }),
  });
}

export interface AgbMetadata extends Metadata {
  type: "AGB";
  color?: string;
  year?: number;
  week?: number;
  mainboard: {
    type: string;
    number_pair?: string;
    stamp?: string;
    circled_letters?: string;
    crystal?: Chip;
    cpu?: Chip;
    work_ram?: Chip;
    regulator?: Chip | null;
    amplifier?: Chip;
    u4?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace AgbMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('AGB'),
    color: Joi.string(),
    year: schemas.year,
    week: schemas.week,
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      number_pair: Joi.string(),
      stamp: Joi.string(),
      circled_letters: Joi.string(),
      crystal: Chip.schema,
      cpu: Chip.schema,
      work_ram: Chip.schema,
      regulator: Chip.schema.allow(null),
      amplifier: Chip.schema,
      u4: Chip.schema,
      year: schemas.year,
      month: schemas.month
    })
  });
}

export interface AgsMetadata extends Metadata {
  type: "AGS";
  mainboard: {
    type: string;
    cpu?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace AgsMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('AGS'),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      cpu: Chip.schema,
      year: schemas.year,
      month: schemas.month,
    })
  });
}

export interface GbsMetadata extends Metadata {
  type: "GBS";
  color?: string;
  year?: number;
  week?: number;
  mainboard: {
    type: string;
    number_pair?: string,
    stamp?: string;
    stamp_front?: string;
    stamp_back?: string;
    circled_letters?: string;
    year?: number;
    month?: number;
    crystal?: Chip;
    cpu?: Chip;
    work_ram?: Chip;
    u4?: Chip;
    u5?: Chip;
    u6?: Chip;
  };
}

export namespace GbsMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('GBS'),
    color: Joi.string(),
    year: schemas.year,
    week: schemas.week,
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      number_pair: Joi.string(),
      stamp: Joi.string(),
      stamp_front: Joi.string(),
      stamp_back: Joi.string(),
      circled_letters: Joi.string(),
      year: schemas.year,
      month: schemas.month,
      crystal: Chip.schema,
      cpu: Chip.schema,
      work_ram: Chip.schema,
      u4: Chip.schema,
      u5: Chip.schema,
      u6: Chip.schema
    })
  });
}

export interface OxyMetadata extends Metadata {
  type: "OXY";
  color?: string;
  mainboard: {
    type: string;
    circled_letters?: string;
    cpu?: Chip;
    u2?: Chip;
    u4?: Chip;
    u5?: Chip;
    year?: number;
    month?: number;
  };
}

export namespace OxyMetadata {
  export const schema = Joi.object().keys({
    type: Joi.string().required().allow('OXY'),
    color: Joi.string(),
    mainboard: Joi.object().required().keys({
      type: Joi.string().required(),
      circled_letters: Joi.string(),
      cpu: Chip.schema,
      u2: Chip.schema,
      u4: Chip.schema,
      u5: Chip.schema,
      year: schemas.year,
      month: schemas.month
    })
  });
}
